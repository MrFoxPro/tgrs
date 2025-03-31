// This is one of most terrible code I wrote

#![allow(unused_imports, unused_must_use)]
#![feature(let_chains)]

mod indent;

use std::{cell::RefCell, collections::{BTreeMap, HashSet, VecDeque}, io::Write, process::Command, rc::Rc};
use anyhow::Result;
use heck::{ToPascalCase, ToSnakeCase};
use indent::IndentedWriter;
use tg_bot_api::{extractor::Extractor, parser, Argument, Field, MethodArgs, ObjectData, Type};

const RESERVED_WORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn",
];

const BLACKLISTED_TYPES: &[&str] = &[
	"Message",
	"MessageEntity",
	"InputFile",
	"Asset",
];

#[derive(Clone, Debug, PartialEq)]
struct SerdeInfo { 
	ser: bool,
	de: bool,
}

#[derive(Clone, Debug, PartialEq)]
enum TypeWrapper {
	Option,
	Vec,
}
#[derive(Clone, Debug, PartialEq)]
struct TypeInfo {
	name: String,
	wrappers: Vec<TypeWrapper>,
	has_ref: bool,
	const_literal: Option<String>,
	maybe_file: bool
}
impl TypeInfo {
	fn is_array(&self) -> bool {
		self.wrappers.get(0).is_some_and(|w| *w == TypeWrapper::Vec)
	}
}

#[derive(Clone, Debug, PartialEq)]
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
impl PartialEq for Entity {
	fn eq(&self, other: &Self) -> bool {
	    self.parents == other.parents && self.comment == other.comment && self.serde == other.serde && self.variant == other.variant
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PayloadKind {
    Multipart,
    Json,
    Empty,
}

#[derive(Clone, Debug, PartialEq)]
enum EntityVariant {
	Unknown,
	Object {
		fields: BTreeMap<String, StructField>,
	},
	Enum {
		internal_tag: Option<&'static str>,
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
	TypeInfo { name, has_ref, wrappers, const_literal: None, maybe_file: ty.maybe_file_to_send() }
}

fn create_enum_variant(enum_name: &String, var: &Type) -> (String, StructField) {
	let TypeInfo { name: var_ty_name, has_ref, maybe_file, .. } = get_plain_typeinfo(&var);
	let mut var_name = var_ty_name.to_pascal_case();
	if enum_name == "ChatId" || enum_name == "FromChatId" {
		if var_name == "String" { var_name = "Username".to_owned(); }
		if var_name == "I64"    { var_name = "Id".to_owned(); }
	}
	else if var_name.len() > enum_name.len() && var_name.starts_with(enum_name) {
		var_name = var_name[enum_name.len()..].to_string();
	}
	(var_name, StructField { 
		name: var_ty_name.clone(), 
		typeinfo: TypeInfo { name: var_ty_name, has_ref, wrappers: Vec::new(), const_literal: None, maybe_file, }, 
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
				return TypeInfo { name: name.clone(), has_ref: true, wrappers: Vec::new(), const_literal: None, maybe_file: target.maybe_file_to_send() }
			}
			let mut variants = vars.iter().map(|var| create_enum_variant(&name, var)).collect::<BTreeMap<_, _>>();
			if variants.contains_key("InputFile") && variants.contains_key("String") {
				variants.remove_entry("InputFile");				
				variants.insert("File".to_string(), StructField { 
					name: "InputFile".into(), 
					typeinfo: TypeInfo { name: "InputFile".into(), has_ref: true, wrappers: Vec::new(), const_literal: None, maybe_file: target.maybe_file_to_send() }, 
					optional: false, 
					comment: String::new()
				});
				variants.remove_entry("String");
				variants.insert("Url".to_string(), StructField { 
					name: "String".into(), 
					typeinfo: TypeInfo { name: "String".into(), has_ref: true, wrappers: Vec::new(), const_literal: None, maybe_file: target.maybe_file_to_send() }, 
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
				variant: EntityVariant::Enum { variants, internal_tag: None },
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
			TypeInfo { has_ref: true, name, wrappers: Vec::new(), const_literal: None, maybe_file:  target.maybe_file_to_send()  }
		},
		Type::Array(t) if matches!(*t.clone(), Type::Or(..)) => {
			let mut inner = expand_typeinfo(state, t, NewEnumInfo { name, parent, serde, comment });
			inner.wrappers.insert(0, TypeWrapper::Vec);
			return inner;
		},
		other => get_plain_typeinfo(&other)
	}
}
	
const CONST_FIELD_PREFIX: &str = "Type of the result, must be ";
fn check_const_literal(ty: &mut TypeInfo, description: &String) {
	if description.starts_with(CONST_FIELD_PREFIX) {
		let name = description[CONST_FIELD_PREFIX.len()+1..description.len()-1].replace(r#"\"#, "").to_string();
		ty.const_literal = name.into();
	}
}

pub fn main() -> Result<()> {
	let source = std::fs::read_to_string("./emitter/source.html").unwrap();
	let parsed = {
		let extractor = Extractor::from_str(&source);
		let extracted = extractor.extract()?;
		parser::parse(extracted)?
	};

	let mut out = IndentedWriter::new(std::fs::File::create("./tg/schema.rs")?, indent::IndentConfig::Tab);

	writeln!(out, "/* @generated */\n");
	writeln!(out, "use serde::{{Serialize, Deserialize}};");
	writeln!(out, "use serde_with::apply;");
	writeln!(out, "use derive_more::{{From, Display}};");
	writeln!(out, "use crate::{{addons::*, custom::*, client::{{Executable, FormParts}}, InputFile}};");
	writeln!(out, "");
	writeln!(out, r#"pub const SCHEMA_VERSION: &str = "{}";"#, parsed.version.to_string());
	writeln!(out, "");

	let mut registry = BTreeMap::new();

	for object in parsed.objects.into_iter() {
		let comment = [object.description.clone(), String::new(), object.docs_link.clone()].join("\n");

		let entity = match object.data {
			ObjectData::Fields(fields) => {
				let mut out_fields = BTreeMap::new();
				for Field { name, kind, required, description } in fields.into_iter() {
					let mut typename_hint = name.to_pascal_case();

					if ["Sticker"].contains(&typename_hint.as_str()) {
						typename_hint = format!("{typename_hint}Input");
					}

					let mut typeinfo = expand_typeinfo(
						&mut registry, 
						&kind, 
						NewEnumInfo { 
							name: typename_hint,
							comment: description.clone().into(),
							parent: Some(name.clone()),
							serde: SerdeInfo { ser: false, de: false }
						}
					);
					if description.contains("attach://") {
						typeinfo.name = "Asset".to_owned();
						typeinfo.wrappers = vec![];
						typeinfo.has_ref = true;
					}

					check_const_literal(&mut typeinfo, &description);

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
				outdef
			}
			ObjectData::Elements(vars) => {
				let variants = vars.iter().map(|var| create_enum_variant(&object.name, var)).collect();

				let internal_tag = match object.name.as_str() {
					"ChatMember" => "status".into(),
					_ => None,
				};
				let outdef = Entity { 
					name: escape_field_name(object.name.clone()), 
					parents: Vec::new(),
					comment,
					serde: SerdeInfo { ser: false, de: false },
					variant: EntityVariant::Enum { variants, internal_tag }
				};
				outdef
			}
			ObjectData::Unknown => {
				let outdef = Entity { 
					name: object.name.clone(), 
					parents: Vec::new(),
					serde: SerdeInfo { ser: false, de: false },
					comment,
					variant: EntityVariant::Unknown,
				};
				outdef				
			}
		};
		if registry.values().any(|e| e.eq(&entity)) {
			println!("skipping duplicating {}", entity.name);
			continue;
		}
		assert!(registry.insert(object.name.clone(), entity).is_none());
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
					let mut typeinfo = expand_typeinfo(
						&mut registry,
						&kind,
						NewEnumInfo { 
							name: typename_hint,
							comment: description.clone(),
							parent: Some(method_name.clone()),
							serde: SerdeInfo { ser: false, de: false }
						}
					);
					check_const_literal(&mut typeinfo, &description);
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
			return_type_info = TypeInfo { name: "EditMessageResult".to_owned(), has_ref: true, wrappers: Vec::new(), const_literal: None, maybe_file: false };
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
			serde: SerdeInfo { ser: false, de: false },
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
			EntityVariant::Object { ref fields } | EntityVariant::Method { args: ref fields, .. } | EntityVariant::Enum { variants: ref fields, .. } => {
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

	let registry_clone = registry.clone();
	for entity in registry.values_mut() {
		match entity.variant {
			EntityVariant::Object { .. } | EntityVariant::Enum { .. } => {
				'c: for centity in registry_clone.values() {
					let EntityVariant::Method { ref args, .. } = centity.variant else { continue };
					for (_, a) in args.iter() {
						if a.typeinfo.name == entity.name {
							entity.serde.ser = true;
							break 'c;
						}
					}
				}
			},
			_ => {}
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
	// 	.arg("./tg/output.rs")
	// 	.stdout(std::io::stdout())
	// 	.stderr(std::io::stderr())
	// 	.output()
	// 	.unwrap();

	Ok(())
}

fn print_entities(registry: Registry, out: &mut IndentedWriter<impl Write>) {
	for entity in registry.values() {

		if BLACKLISTED_TYPES.contains(&entity.name.as_str()) {
			continue;
		}
		// let doc = format!("{} {}", match entity.variant { EntityVariant::Object { .. } => "object", EntityVariant::Method { .. } => "method", _ => "" }, entity.doc);
		if entity.comment.len() > 0 { writeln!(out, "/**{}*/", entity.comment); }

		match entity.variant.clone() {
			EntityVariant::Unknown => {
				writeln!(out, "pub type {} = ();", entity.name);
			}
			EntityVariant::Object { fields } => {
				print_struct(&registry, &entity, &fields, out);
			}
			EntityVariant::Method { kind, args, return_type, api_name } => {
				print_struct(&registry, &entity, &args, out);

				writeln!(out, "impl Executable for {} {{", entity.name);
				out.indent();

				writeln!(out, "type Response = {};", field_typename(&StructField { typeinfo: return_type, name: String::new(), optional: false, comment: String::new() }, entity));
				writeln!(out, r#"const METHOD_NAME: &str = "{}";"#, api_name);
				writeln!(out, "fn into_parts(self) -> FormParts {{");
				out.indent();

				let mut capacity = 0;
				for arg in args.values() {
					 if ["InputFile", "Asset"].contains(&arg.name.as_str()) { capacity += 2; } 
					 else { capacity += 1 }
				}

				if !args.is_empty() {
					writeln!(out, "let mut parts = FormParts::new({capacity});");

					for (name, sf) in args.iter() {
						if sf.typeinfo.is_array() && !sf.typeinfo.maybe_file {
							writeln!(out, r#"if self.{name}.len() > 0 {{ parts.add_object("{name}", self.{name}) }}"#);
						}
						else if ["String", "i64", "f32", "bool"].contains(&sf.typeinfo.name.as_str()) {
							writeln!(out, r#"parts.add_{}("{name}", self.{name});"#, sf.typeinfo.name.to_snake_case());
						}
						else if let Some(e) = registry.get(&sf.typeinfo.name) && let EntityVariant::Enum { ref variants, .. } = e.variant && variants.iter().all(|(vname, vfield)| !vfield.typeinfo.has_ref) {
							if sf.optional {
								writeln!(out, r#"parts.add_string("{name}", self.{name}.map(|x| x.to_string()));"#);
							}
							else {
								writeln!(out, r#"parts.add_string("{name}", self.{name}.to_string());"#);
							}
						}
						else {
							if sf.optional {
								write!(out, r#"if let Some({name}) = self.{name} {{ "#);
							}

							if sf.typeinfo.maybe_file {
								write!(out, r#"parts.add_attachable"#);
							}
							else {
								write!(out, r#"parts.add_object"#);
							}

							write!(out, r#"("{name}", "#);
							if !sf.optional { write!(out, "self.{name}"); } else { write!(out, "{name}"); }  
							write!(out, ");");

							if sf.optional {
								write!(out, r#" }}"#);
							}

							writeln!(out, "");
						}


						// if sf.optional && !sf.typeinfo.is_array() { 
						// 	writeln!(out, "if let Some({name}) = self.{name} {{");
						// 	out.indent();

						// 	writeln!(out, r#"parts.push(("{name}".into(), {part}));"#); 

						// 	out.unindent();
						// 	writeln!(out, "}}");
						// }
						// else if sf.typeinfo.is_array() {
						// 	writeln!(out, r#"if !self.{name}.is_empty() {{"#);
						// 	out.indent();
							
						// 	writeln!(out, r#"parts.push(("{name}".into(), {part}));"#);
							
						// 	out.unindent();
						// 	writeln!(out, "}}");
						// }
						// else { writeln!(out, r#"parts.push(("{name}".into(), {part}));"#); }
					}
					writeln!(out, r#"parts"#);
				}
				else {
					writeln!(out, r#"FormParts::new(0)"#);
				}
				out.unindent();
				writeln!(out, "}}");

				out.unindent();
				writeln!(out, "}}");

				// let suffix = match kind { PayloadKind::Empty => "empty", PayloadKind::Multipart => "multipart", PayloadKind::Json => "json" };

				// writeln!(out, r#"method!({}, "{}", {});"#, entity.name, api_name, return_type.name);
			}
			EntityVariant::Enum { variants, internal_tag } => {
				print_derive(&entity, out);
				if entity.serde.ser || entity.serde.de { 
					if let Some(internal_tag) = internal_tag {
						writeln!(out, r#"#[serde(tag = "{internal_tag}", rename_all = "snake_case")]"#);
					}
					else {
						writeln!(out, r#"#[serde(untagged, rename_all = "snake_case")]"#);
					}
				}
				writeln!(out, "pub enum {} {{", entity.name);
				out.indent();
				// writeln!(out, "#[default]");
				for (varname, vartype) in variants.into_iter() {
					let mut vartypename = vartype.name;
					if entity.name == "MaybeInaccessibleMessage" && vartypename == "Message" {
						vartypename = format!("Box<{}>", vartypename);
					}
					if entity.name == "ChatMember" {
						match varname.as_str() {
							"Banned" => { writeln!(out, r#"#[serde(rename = "kicked")]"#); }
							"Owner"  => { writeln!(out, r#"#[serde(rename = "creator")]"#); }
							_ => {}
						}
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

	if let EntityVariant::Enum { ref variants, internal_tag, .. } = entity.variant  {
		if !variants.contains_key("File") && !variants.contains_key("Url") {
			derives.push("From");
		}
		if variants.iter().all(|(vname, vfield)| !vfield.typeinfo.has_ref) {
			derives.push("Display");
		}
	}

	writeln!(out, "#[derive({})]", derives.join(", "));	
}

fn field_typename(field: &StructField, root: &Entity) -> String {
	if field.typeinfo.const_literal.is_some() {
		return String::from("&'static str")
	}
    let mut typename = field.typeinfo.name.clone();
    // Workarounds
	if root.name == "Message" && field.name == "reply_to_message" && typename == "Message" {
		typename = format!("Box<{}>", field.typeinfo.name);
	}
	if root.name == "GiveawayCompleted" && field.name == "giveaway_message" && typename == "Message" {
		typename = format!("Box<{}>", field.typeinfo.name);
	}

	if !field.typeinfo.is_array() && field.optional { 
		typename = format!("Option<{typename}>");
	}
	else if field.typeinfo.is_array() {
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

fn print_struct(registry: &Registry, entity: &Entity, fields: &BTreeMap<String, StructField>, out: &mut IndentedWriter<impl Write>) {
	let (mut has_vecs, mut has_opts) = (false, false);
	fields.values().for_each(|f| {
		if f.optional && !f.typeinfo.is_array() { has_opts = true }
		if f.typeinfo.is_array() { has_vecs = true }
	});

	if (entity.serde.ser || entity.serde.de) && (has_vecs || has_opts) {
		writeln!(out, "#[apply(");
		out.indent();
		if has_vecs {
			writeln!(out, r#"Vec => #[serde(skip_serializing_if = "Vec::is_empty", default)],"#);
		}
		if has_opts {
			writeln!(out, r#"Option => #[serde(skip_serializing_if = "Option::is_none")],"#);
		}
		out.unindent();
		writeln!(out, ")]");
	}

	print_derive(&entity, out);

	write!(out, "pub struct {}", entity.name);
	// if fields.len() > 0 { 
		writeln!(out, " {{"); 
		out.indent();
	// } 
	// else { 
		// writeln!(out, ";");
	// }
	let mut to_skip = Vec::new();
	for field in fields.clone().into_values() {
		let parents = entity.parents.iter().filter_map(|parent| registry.get(parent)).collect::<Vec<_>>();
		if parents.into_iter().any(|parent| { match parent.variant { EntityVariant::Enum { internal_tag: Some(tag), .. } => tag == field.name, _ => false } }) {
			println!("skipping field {}.{} as it is tag", entity.name, field.name);
			to_skip.push(field.name);
			continue;
		}

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
	// if fields.len() > 0 {
		// if entity.serde.ser || entity.serde.de {
		// 	write!(out, "#[serde(skip)] ");
		// }
		// writeln!(out, "pub parts: FormParts,");
		out.unindent();
		write!(out, "}}");
	// }
	// else { return }

	// if !entity.serde.ser {
		// writeln!(out, "");
		// return
	// }

	struct ConstructorArg {
		into: bool
	}
	let ctor_args = fields
		.values()
		.map(|field| (field, ConstructorArg {
			into: !field.typeinfo.const_literal.is_some() && !["bool"].contains(&field.typeinfo.name.as_str())
		}))
		.collect::<Vec<_>>();

	writeln!(out, "\nimpl {} {{", entity.name);
	out.indent();
		write!(out, "pub fn new(");
		let new_args = ctor_args.iter().filter(|(f, _)| !f.optional && f.typeinfo.const_literal.is_none() && !to_skip.contains(&f.name)).collect::<Vec<_>>();
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
				if to_skip.contains(&f.name) { continue; }
				let mut v: String;
				if let Some(ref const_value) = f.typeinfo.const_literal {
					v = format!(r#""{}""#, const_value);
				}
				else if !f.optional { v = f.name.clone(); }
				else {
					if f.typeinfo.is_array() { v = "Vec::new()".to_owned(); }
					else { v = "None".to_owned(); }
				}
				if !f.optional && a.into { v = format!("{}.into()", v); }
				writeln!(out, "{}: {},", f.name, v);
			}
			// writeln!(out, "parts: FormParts::new(),");
			out.unindent();
			writeln!(out, "}}");

		out.unindent();
		writeln!(out, "}}");
		
	for (field, arg) in ctor_args.into_iter() {
		if field.typeinfo.is_array() {
			let pluralized_field_name = pluralizer::pluralize(&field.name, 1, false);
			write!(out, "pub fn add_{0}(mut self, {0}: ", pluralized_field_name);
	 		let accept_type = field_typename(&StructField { 
	 			optional: false, 
	 			typeinfo: TypeInfo { 
	 				wrappers: field.typeinfo.wrappers.clone().into_iter().skip(1).collect(),
	 				..field.typeinfo.clone() 
	 			}, 
	 			..field.clone() 
	 		}, entity);
	 		if arg.into { write!(out, "impl Into<{}>", accept_type); }
	 		else { write!(out, "{}", accept_type); }
			writeln!(out, ") -> Self {{");

			out.indent();
			write!(out, "self.{}.push(", field.name);
			if arg.into && !field.typeinfo.is_array() {
				write!(out, "Some({}.into())", pluralized_field_name);
			}
			else if field.typeinfo.is_array() {
				write!(out, "{}.into()", pluralized_field_name);
			}
			else {
				write!(out, "Some({})", pluralized_field_name);
			}
			writeln!(out, ");");
			writeln!(out, "self");

			out.unindent();
			writeln!(out, "}}");
		}


		if !field.optional { continue; }
		writeln!(out, "/** {}*/", field.comment);
		write!(out, "pub fn {0}(mut self, {0}: ", field.name);

 		let accept_type = field_typename(&StructField { optional: false, ..field.clone() }, entity);
 		if arg.into { write!(out, "impl Into<{}>", accept_type); }
 		else { write!(out, "{}", accept_type); }
		writeln!(out, ") -> Self {{");

		out.indent();
		write!(out, "self.{} = ", field.name);
		if arg.into && !field.typeinfo.is_array() {
			write!(out, "Some({}.into())", field.name);
		}
		else if field.typeinfo.is_array() {
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

// macro_rules! method {
// 	($name:ty, $id:literal, $response:ty) => {
// 		impl Executable for $name {
// 			type Response = $response;
// 			fn method_name(&self) -> &'static str { $id }
// 			fn parts(&self) -> &FormParts { &self.parts }
// 			fn parts_mut(&mut self) -> &mut FormParts { &mut self.parts }
// 			fn into_parts(self) {
// 				let mut parts = Vec::with_capacity(32);
// 				// Serialize::serialize(&self, &mut PartsSerializer::new(&mut parts)).unwrap();
// 				self.parts_mut().extend(parts);
// 			}
// 		}
// 	};
// }
