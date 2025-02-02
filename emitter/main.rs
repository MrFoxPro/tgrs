#![allow(unused_must_use)]
#![feature(let_chains)]

mod indent;

use std::{cell::RefCell, collections::{BTreeMap, HashSet, VecDeque}, io::Write, process::Command, rc::Rc};
use anyhow::Result;
use heck::ToPascalCase;
use indent::IndentedWriter;
use tg_bot_api::{extractor::Extractor, parser, Argument, Field, MethodArgs, ObjectData, Type};

const RESERVED_WORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn",
];

const BLACKLISTED_TYPES: &[&str] = &[
	// "ChatId",
	"InputFile",
];

#[derive(Clone, Debug)]
struct SerdeInfo { 
	ser: bool,
	de: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum TypeWrapper {
	Option,
	Vec,
}
#[derive(Clone, Debug)]
struct TypeInfo {
	name: String,
	wrappers: Vec<TypeWrapper>,
	has_ref: bool,
}
impl TypeInfo {
	fn array(&self) -> bool {
		self.wrappers.get(0).is_some_and(|w| *w == TypeWrapper::Vec)
	}
}

#[derive(Clone, Debug)]
struct StructField {
	typeinfo: TypeInfo,
	name: String,
	optional: bool,
	comment: String,
}

#[derive(Clone, Debug)]
struct Entity {
	name: String,
	parents: Vec<String>,
	comment: String,
	serde: SerdeInfo,
	variant: EntityVariant,
}

#[derive(Clone, Debug)]
pub enum PayloadKind {
    Multipart,
    Json,
    Empty,
}

#[derive(Clone, Debug)]
enum EntityVariant {
	Unknown,
	Object {
		fields: BTreeMap<String, StructField>,
	},
	Enum {
		variants: BTreeMap<String, StructField>,
	},
	Method {
		api_name: String,
		args: BTreeMap<String, StructField>,
		return_type: TypeInfo,
		kind: PayloadKind
	},
}

type Registry = BTreeMap<String, Entity>;

fn escape_field_name(mut name: String) -> String {
	if RESERVED_WORDS.contains(&name.as_str()) {
		name = format!("r#{name}");
	}
	return name
}

fn get_plain_typeinfo(ty: &Type) -> TypeInfo {
	let mut has_ref = false;
	let mut wrappers = Vec::new();
	let name = match ty {
		Type::String  { .. } 	=> "String".into(),
		Type::Integer { .. } 	=> "i64".into(),
		Type::Float   { .. }	=> "f32".into(),
		Type::Bool    { .. } 	=> "bool".into(),
		Type::Array(t) => {
			let arr_typeinfo = get_plain_typeinfo(t);

			has_ref = arr_typeinfo.has_ref;
			wrappers.insert(0, TypeWrapper::Vec);
			wrappers.extend(arr_typeinfo.wrappers);

			arr_typeinfo.name
		},
		Type::Object(obj_name) => {
			has_ref = true;
			obj_name.clone()
		},
		_ => panic!("{ty:?} is not a plain type")
	};
	TypeInfo { name, has_ref, wrappers }
}

fn create_enum_variant(enum_name: &String, var: &Type) -> (String, StructField) {
	let TypeInfo { name: var_ty_name, .. } = get_plain_typeinfo(&var);
	let mut var_name = var_ty_name.to_pascal_case();
	if enum_name == "ChatId" || enum_name == "FromChatId" {
		if var_name == "String" { var_name = "Username".to_owned(); }
		if var_name == "I64" { var_name = "Id".to_owned(); }
	}
	else if var_name.len() > enum_name.len() && var_name.starts_with(enum_name) {
		var_name = var_name[enum_name.len()..].to_string();
	}
	(var_name, StructField { 
		name: var_ty_name.clone(), 
		typeinfo: TypeInfo { name: var_ty_name, has_ref: true, wrappers: Vec::new() }, 
		optional: false, 
		comment: String::new()
	}) 
}

struct NewEnumInfo { 
	name: String, 
	comment: String, 
	parent: Option<String>, 
	serde: SerdeInfo 
}
fn expand_typeinfo(state: &mut Registry, target: &Type, NewEnumInfo { mut name, mut comment, parent, serde }: NewEnumInfo) -> TypeInfo {
	match target {
		Type::Or(vars) => {
			// create new enum for "Or" type
			// assuming that it is not recursive (does not contain inner `Or`s) 
			if BLACKLISTED_TYPES.contains(&name.as_str()) {
				println!("skipped blacklisted type {name}");
				return TypeInfo { name: name.clone(), has_ref: true, wrappers: Vec::new() }
			}
			let mut variants = vars.iter().map(|var| create_enum_variant(&name, var)).collect::<BTreeMap<_, _>>();
			if variants.contains_key("InputFile") && variants.contains_key("String") {
				variants.remove_entry("InputFile");				
				variants.insert("File".to_string(), StructField { 
					name: "InputFile".into(), 
					typeinfo: TypeInfo { name: "InputFile".into(), has_ref: true, wrappers: Vec::new() }, 
					optional: false, 
					comment: String::new()
				});
				variants.remove_entry("String");
				variants.insert("Url".to_string(), StructField { 
					name: "String".into(), 
					typeinfo: TypeInfo { name: "String".into(), has_ref: true, wrappers: Vec::new() }, 
					optional: false, 
					comment: String::new()
				});
				name = "Asset".to_owned();
				comment = "".to_owned();
			}

			let def = Entity {
				name: name.clone(),
				serde,
				parents: parent.map(|p| Vec::from_iter([p])).unwrap_or(Vec::new()),
				variant: EntityVariant::Enum { variants },
				comment: comment.clone(),
			};
			if let Some(pdef) = state.insert(name.clone(), def.clone()) {
				// TODO compare actual implementations
				// if format!("{:?}", pdef.variant) != format!("{:?}", def.variant) && !["ChatId", "FromChatId", "ThumbnailArg"].contains(&pdef.name.as_str()) { 
				// 	eprintln!("different types:");
				// 	eprintln!("{}\n{}\n{:#?}", pdef.name, pdef.doc, pdef.variant);
				// 	eprintln!("{}\n{}\n{:#?}", def.name, def.doc, def.variant);
				// 	panic!();
				// }
			};
			TypeInfo { has_ref: true, name, wrappers: Vec::new() }
		},
		Type::Array(t) if matches!(*t.clone(), Type::Or(..)) => {
			expand_typeinfo(state, t, NewEnumInfo { name, parent, serde, comment })
		},
		other => get_plain_typeinfo(&other)
	}
}

pub fn main() -> Result<()> {
	let source = std::fs::read_to_string("./gen/source.html").unwrap();
	let parsed = {
		let extractor = Extractor::from_str(&source);
		let extracted = extractor.extract()?;
		parser::parse(extracted)?
	};

	let mut out = IndentedWriter::new(std::fs::File::create("./types/output.rs")?, indent::IndentConfig::Tab);

	writeln!(out, "/* @generated */\n");
	writeln!(out, "use serde::{{Serialize, Deserialize}};");
	writeln!(out, "use serde_with::apply;");
	writeln!(out, "use derive_more::From;");
	writeln!(out, "use crate::{{custom::*, method, InputFile}};");
	writeln!(out, "");

	let mut registry = BTreeMap::new();

	for object in parsed.objects.into_iter() {
		let comment = [object.description.clone(), String::new(), object.docs_link.clone()].join("\n");

		match object.data {
			ObjectData::Fields(fields) => {
				let mut out_fields = BTreeMap::new();
				for Field { name, kind, required, description } in fields.into_iter() {
					let mut typename_hint = name.to_pascal_case();

					if ["Sticker"].contains(&typename_hint.as_str()) {
						typename_hint = format!("{typename_hint}Input");
					}

					let typeinfo = expand_typeinfo(
						&mut registry, 
						&kind, 
						NewEnumInfo { 
							name: typename_hint,
							comment: description.clone().into(),
							parent: Some(name.clone()),
							serde: SerdeInfo { ser: false, de: false }
						}
					);

					let mut docs = Vec::from_iter([description]);
					match kind.clone() {
						Type::String { default, min_len, max_len, one_of } => {
							if !one_of.is_empty() {
								docs.push(format!("One of: {}", one_of.join(", ")));
							}
							if let Some(default) = default {
								docs.push(format!("Default: {}", default));
							}
							if let Some(min_len) = min_len {
								docs.push(format!("Min len: {}", min_len));
							}
							if let Some(max_len) = max_len {
								docs.push(format!("Max len: {}", max_len));
							}
						}
						Type::Integer { default, min, max, one_of } => {
							if !one_of.is_empty() {
								let s = one_of.into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ");
								docs.push(format!("One of: {}", s)); 
							}
							if let Some(default) = default {
								docs.push(format!("Default value: {}", default));
							}
							if let Some(min) = min {
								docs.push(format!("Min: {}", min));
							}
							if let Some(max) = max {
								docs.push(format!("Max: {}", max));
							}
						}
						Type::Float {} => {}
						Type::Bool { default } => {
							if let Some(default) = default {
								docs.push(format!("Default value: {}", default));
							}
						}
						_ => {}
					};
					let sfield = StructField { 
						typeinfo, 
						name: escape_field_name(name),
						optional: !required,
						comment: docs.join("\n")
					};
					out_fields.insert(sfield.name.clone(), sfield);
				}
				let outdef = Entity { 
					name: object.name.to_pascal_case(), 
					parents: Vec::new(),
					comment,
					serde: SerdeInfo { ser: false, de: false },
					variant: EntityVariant::Object { fields: out_fields }
				};
				assert!(registry.insert(object.name.clone(), outdef).is_none());
			}
			ObjectData::Elements(vars) => {
				let variants = vars.iter().map(|var| create_enum_variant(&object.name, var)).collect();
				let outdef = Entity { 
					name: escape_field_name(object.name.clone()), 
					parents: Vec::new(),
					comment,
					serde: SerdeInfo { ser: false, de: false },
					variant: EntityVariant::Enum { variants }
				};
				assert!(registry.insert(object.name.clone(), outdef).is_none());
			}
			ObjectData::Unknown => {
				let outdef = Entity { 
					name: object.name.clone(), 
					parents: Vec::new(),
					serde: SerdeInfo { ser: false, de: false },
					comment,
					variant: EntityVariant::Unknown,
				};
				if !["InputFile"].contains(&outdef.name.as_str()) {
					assert!(registry.insert(object.name.clone(), outdef).is_none());
				}
			}
		}
	}

	for method in parsed.methods.into_iter() {
		let mut fields = BTreeMap::new();
		let method_name = method.name.to_pascal_case();
		let method_kind = match method.args { 
			MethodArgs::WithMultipart(..) => PayloadKind::Multipart,
			MethodArgs::Yes(..) => PayloadKind::Json,
			MethodArgs::No => PayloadKind::Empty,
		};

		match method.args {
			MethodArgs::WithMultipart(args) | MethodArgs::Yes(args) => {
				for Argument { name, kind, description, required } in args.into_iter() {
					let mut typename_hint = name.to_pascal_case();
					// if state.registry.contains_key(&typename_hint) {
						// typename_hint = format!("{typename_hint}");
					// }
					if ["Audio", "Thumbnail", "Document", "Video", "Animation", "Voice", "VideoNote", "Sticker"].contains(&typename_hint.as_str()) {
						typename_hint = format!("{typename_hint}Arg");
					}
					let typeinfo = expand_typeinfo(
						&mut registry,
						&kind,
						NewEnumInfo { 
							name: typename_hint,
							comment: description.clone(),
							parent: Some(method_name.clone()),
							serde: SerdeInfo { ser: true, de: false }
						}
					);
					let def = StructField {
						name: escape_field_name(name.clone()), 
						typeinfo,
						optional: !required,
						comment: description,
					};
					assert!(fields.insert(def.name.clone(), def).is_none());
				}
			}
			_ => {}
		}

		let return_type_info;
		if ["EditMessageCaption", "EditMessageLiveLocation", "EditMessageMedia", "EditMessageReplyMarkup", "EditMessageText"].contains(&method_name.as_str()) {
			return_type_info = TypeInfo { name: "EditMessageResult".to_owned(), has_ref: true, wrappers: Vec::new() };
		}
		else {
			return_type_info = expand_typeinfo(
				&mut registry,
				&method.return_type,
				NewEnumInfo { 
					name: format!("{method_name}Result"),
					serde: SerdeInfo { ser: false, de: true },
					parent: Some(method_name.clone()),
					comment: format!("{} return value", method.name),
				}
			);
		}

		let entity = Entity { 
			name: method_name, 
			parents: Vec::new(),
			serde: SerdeInfo { ser: true, de: false },
			variant: EntityVariant::Method { 
				kind: method_kind,
				api_name: method.name.clone(),
				args: fields, 
				return_type: return_type_info,
			},
			comment: [method.description.clone(), String::new(), method.docs_link.clone()].join("\n"),
		};

		assert!(registry.insert(method.name.clone(), entity).is_none());
	}

	// normalize parents
	let registry_clone = registry.clone();
	for entity in registry_clone.values() {	
		match entity.variant {
			EntityVariant::Object { ref fields } | EntityVariant::Method { args: ref fields, .. } | EntityVariant::Enum { variants: ref fields } => {
				for f in fields.values() {
					if let Some(fentity) = registry.get_mut(&f.typeinfo.name) && !fentity.parents.contains(&entity.name) {
						fentity.parents.push(entity.name.clone());
						fentity.serde.ser |= entity.serde.ser; 
						fentity.serde.de |= entity.serde.de; 
					}
				}
			},
			_ => {}
		}

		if let EntityVariant::Method { ref return_type, .. } = entity.variant {
			if !return_type.has_ref { continue; }
			if let Some(rtype_entity) = registry.get_mut(&return_type.name) && !rtype_entity.parents.contains(&entity.name) {
				rtype_entity.serde.de = true;
				rtype_entity.parents.push(entity.name.clone());
			}
		}
	}

	let registry = registry.into_iter().map(|(k, v)| (k, Rc::new(RefCell::new(v)))).collect::<BTreeMap<_, _>>();
	for entity in registry.values() {
		let mut entity = entity.borrow_mut();

		let mut stack = VecDeque::from_iter(entity.parents.clone());
		let mut history = HashSet::new();

		while let Some(item) = stack.pop_front() {
			if !history.insert(item.clone()) { continue };

			let Some(pentity) = registry.get(&item).and_then(|x| x.try_borrow().ok()) else { continue; };

			entity.serde.ser |= pentity.serde.ser;
			entity.serde.de |= pentity.serde.de;

			for p in pentity.parents.iter() {
				stack.push_back(p.clone());
			}
		}
	}

	let registry = registry.into_iter().map(|(k, v)| (k, Rc::try_unwrap(v).unwrap().into_inner())).collect();
	print_entities(registry, &mut out);

	// Command::new("rustfmt")
	// 	.arg("--config-path=./rustfmt.toml")
	// 	.arg("--emit=files")
	// 	.arg("./types/output.rs")
	// 	.stdout(std::io::stdout())
	// 	.stderr(std::io::stderr())
	// 	.output()
	// 	.unwrap();

	Ok(())
}

fn print_entities(registry: Registry, out: &mut IndentedWriter<impl Write>) {
	for entity in registry.values() {
		// let doc = format!("{} {}", match entity.variant { EntityVariant::Object { .. } => "object", EntityVariant::Method { .. } => "method", _ => "" }, entity.doc);
		if entity.comment.len() > 0 { writeln!(out, "/**{}*/", entity.comment); }

		match entity.variant.clone() {
			EntityVariant::Unknown => {
				writeln!(out, "type {} = ();", entity.name);
			}
			EntityVariant::Object { fields } => {
				print_struct(&entity, &fields, out);
			}
			EntityVariant::Method { kind, args, return_type, api_name } => {
				print_struct(&entity, &args, out);
				// let suffix = match kind { PayloadKind::Empty => "empty", PayloadKind::Multipart => "multipart", PayloadKind::Json => "json" };
				writeln!(out, r#"method!({}, "{}", {});"#, entity.name, api_name, return_type.name);
			}
			EntityVariant::Enum { variants } => {
				print_derive(&entity, out);
				writeln!(out, "#[serde(untagged)]");
				writeln!(out, "pub enum {} {{", entity.name);
				out.indent();
				// writeln!(out, "#[default]");
				for (varname, vartype) in variants.into_iter() {
					let mut vartypename = vartype.name;
					if entity.name == "MaybeInaccessibleMessage" && vartypename == "Message" {
						vartypename = format!("Box<{}>", vartypename);
					}
					writeln!(out, "{varname}({}),", vartypename);
				}
				out.unindent();
				writeln!(out, "}}");	
			}
		}
	}
}

fn print_derive(entity: &Entity, out: &mut IndentedWriter<impl Write>) {
	let mut derives = Vec::from_iter([
		"Clone",
		"Debug",
		// "Default"
	]);
	if entity.serde.ser { derives.push("Serialize"); }
	if entity.serde.de { derives.push("Deserialize"); }

	if let EntityVariant::Enum { ref variants } = entity.variant  {
		if !variants.contains_key("File") && !variants.contains_key("Url") {
			derives.push("From");
		}
	}

	writeln!(out, "#[derive({})]", derives.join(", "));	
}

fn field_typename(field: &StructField, root: &Entity) -> String {
    let mut typename = field.typeinfo.name.clone();
    // Workarounds
	if root.name == "Message" && field.name == "reply_to_message" && typename == "Message" {
		typename = format!("Box<{}>", field.typeinfo.name);
	}
	if root.name == "GiveawayCompleted" && field.name == "giveaway_message" && typename == "Message" {
		typename = format!("Box<{}>", field.typeinfo.name);
	}

	if !field.typeinfo.array() && field.optional { 
		typename = format!("Option<{typename}>");
	}
	else if field.typeinfo.array() {
		for wrapper in field.typeinfo.wrappers.iter() {
			match wrapper {
				TypeWrapper::Vec => {
					typename = format!("Vec<{typename}>");
				}
				_ => unimplemented!()
			}
		}
	}

	return typename
}

fn print_struct(entity: &Entity, fields: &BTreeMap<String, StructField>, out: &mut IndentedWriter<impl Write>) {
	let (mut has_vecs, mut has_opts) = (false, false);
	fields.values().for_each(|f| {
		if f.optional && !f.typeinfo.array() { has_opts = true }
		if f.typeinfo.array() { has_vecs = true }
	});

	if (entity.serde.ser || entity.serde.de) && (has_vecs || has_opts) {
		writeln!(out, "#[apply(");
		out.indent();
		if has_vecs {
			writeln!(out, r#"Vec => #[serde(skip_serializing_if = "Vec::is_empty")],"#);
		}
		if has_opts {
			writeln!(out, r#"Option => #[serde(skip_serializing_if = "Option::is_none")],"#);
		}
		out.unindent();
		writeln!(out, ")]");
	}

	print_derive(&entity, out);

	write!(out, "pub struct {}", entity.name);
	if fields.len() > 0 { 
		writeln!(out, " {{"); 
		out.indent();
	} 
	else { 
		writeln!(out, ";");
	}
	for field in fields.clone().into_values() {
		let mut comment = field.comment.clone();
		if field.optional { 
			comment = comment.replace("*Optional*. ", ""); 
		}
		if field.typeinfo.name == "bool" { 
			comment = comment.replace("*True*, if", "if");
		}
		writeln!(out, "/**{comment}*/");

		let typename = field_typename(&field, entity);
		writeln!(out, "pub {}: {},", field.name, typename);
	}
	if fields.len() > 0 {
		out.unindent();
		write!(out, "}}");
	}
	else { return }

	struct ConstructorArg {
		into: bool
	}
	let ctor_args = fields
		.values()
		.map(|f| (f, ConstructorArg {
			into: !["bool"].contains(&f.typeinfo.name.as_str())
		}))
		.collect::<Vec<_>>();

	writeln!(out, "\nimpl {} {{", entity.name);
	out.indent();
		write!(out, "pub fn new(");
		let new_args = ctor_args.iter().filter(|(f, _)| !f.optional).collect::<Vec<_>>();
		let total = new_args.len();
		for (i, (field, arg)) in new_args.into_iter().enumerate() {
			write!(out, "{}: ", field.name);
			if arg.into {
				write!(out, "impl Into<{}>", field_typename(field, entity));
			}
			else {
				write!(out, "{}", field_typename(field, entity));
			}
			if i < total - 1 { write!(out, ", "); }
		}
		writeln!(out, ") -> Self {{");
		out.indent();

			writeln!(out, "Self {{");
			out.indent();
			for (f, a) in ctor_args.iter() {
				let mut v = String::new();
				if !f.optional { v = f.name.clone(); }
				else {
					if f.typeinfo.array() { v = "Vec::new()".to_owned(); }
					else { v = "None".to_owned(); }
				}
				if !f.optional && a.into { v = format!("{}.into()", v); }
				writeln!(out, "{}: {},", f.name, v);
			}
			out.unindent();
			writeln!(out, "}}");

		out.unindent();
		writeln!(out, "}}");
		
	for (field, arg) in ctor_args.iter() {
		if !field.optional { continue; }
		write!(out, "pub fn {0}(mut self, {0}: ", field.name);

 		let accept_type = field_typename(&StructField { optional: false, ..field.clone().clone() }, entity);
 		if arg.into { write!(out, "impl Into<{}>", accept_type); }
 		else { write!(out, "{}", accept_type); }
		writeln!(out, ") -> Self {{");

		out.indent();
		write!(out, "self.{} = ", field.name);
		if arg.into && !field.typeinfo.array() {
			write!(out, "Some({}.into())", field.name);
		}
		else if field.typeinfo.array() {
			write!(out, "{}.into()", field.name);
		}
		else {
			write!(out, "Some({})", field.name);
		}

		writeln!(out, ";");
		writeln!(out, "self");

		out.unindent();
		writeln!(out, "}}");
	}

	out.unindent();
	writeln!(out, "}}");
}