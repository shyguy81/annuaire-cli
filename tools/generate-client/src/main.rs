use anyhow::{Context, Result};
use clap::Parser;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

struct ClientConfig {
    name: &'static str,
    spec_url: &'static str,
    spec_file: &'static str,
    client_file: &'static str,
    types_file: &'static str,
    tools_file: Option<&'static str>,
}

const CONFIGS: &[ClientConfig] = &[ClientConfig {
    name: "Annuaire",
    spec_url: "https://annuaire-api.demo.docker.dev/openapi.json",
    spec_file: "annuaire-api.json",
    client_file: "client.rs",
    types_file: "types.rs",
    tools_file: Some("shared/src/generated/mcp_tools.rs"),
}];

const RUST_KEYWORDS: &[&str] = &[
    "type", "ref", "move", "unsafe", "async", "await", "dyn", "static", "const", "mut", "pub",
    "use", "crate", "super", "self",
];

#[derive(Parser, Debug)]
#[command(name = "generate-client")]
#[command(about = "Génère les clients API Rust à partir des specs OpenAPI")]
struct Args {
    #[arg(long)]
    fetch_only: bool,
    #[arg(long)]
    generate_only: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.fetch_only && args.generate_only {
        anyhow::bail!("❌ Impossible d'utiliser --fetch-only et --generate-only simultanément");
    }

    println!("🔨 Génération des clients API Annuaire");

    let specs_dir = PathBuf::from(".specs");
    fs::create_dir_all(&specs_dir).context("Erreur création dossier .specs")?;

    let output_dir = PathBuf::from("shared/src/generated");
    fs::create_dir_all(&output_dir).context("Erreur création dossier shared/src/generated")?;

    if args.fetch_only {
        println!("\n📥 Téléchargement des specs OpenAPI...");
        for config in CONFIGS {
            fetch_and_save_spec(config, &specs_dir).await?;
        }
        println!("\n✅ Specs téléchargées avec succès");
        return Ok(());
    }

    if args.generate_only {
        println!("\n🔨 Génération des clients Rust...");
        for config in CONFIGS {
            let spec_path = specs_dir.join(config.spec_file);
            if !spec_path.exists() {
                anyhow::bail!(
                    "❌ Spec {} non trouvée. Exécutez d'abord : ./scripts/generate-client.sh --fetch-only",
                    spec_path.display()
                );
            }
            generate_from_config(config, &specs_dir, &output_dir)?;
        }
        update_mod_file(&output_dir)?;
        println!("\n✅ Clients générés avec succès");
        return Ok(());
    }

    // Default: fetch + generate
    println!("\n📥 Étape 1 : Téléchargement des specs OpenAPI...");
    for config in CONFIGS {
        fetch_and_save_spec(config, &specs_dir).await?;
    }

    println!("\n🔨 Étape 2 : Génération des clients Rust...");
    for config in CONFIGS {
        generate_from_config(config, &specs_dir, &output_dir)?;
    }
    update_mod_file(&output_dir)?;

    println!("\n✅ Tous les clients générés avec succès");
    Ok(())
}

async fn fetch_and_save_spec(config: &ClientConfig, specs_dir: &PathBuf) -> Result<()> {
    let spec_path = specs_dir.join(config.spec_file);
    println!("  📥 {} - Téléchargement depuis {}", config.name, config.spec_url);

    let http_client = reqwest::Client::new();
    let spec_content = http_client
        .get(config.spec_url)
        .header("Cache-Control", "no-cache")
        .send()
        .await
        .context(format!("Erreur requête spec {}", config.name))?
        .text()
        .await
        .context(format!("Erreur lecture spec {}", config.name))?;

    let _: Value = serde_json::from_str(&spec_content)
        .context(format!("Erreur parse JSON spec {}", config.name))?;

    fs::write(&spec_path, &spec_content)
        .context(format!("Erreur sauvegarde spec {}", config.name))?;

    println!("  ✅ Spec sauvegardée : {}", spec_path.display());
    Ok(())
}

fn generate_from_config(
    config: &ClientConfig,
    specs_dir: &PathBuf,
    output_dir: &PathBuf,
) -> Result<()> {
    let spec_path = specs_dir.join(config.spec_file);
    println!("\n  📖 {} - Lecture spec depuis {}", config.name, spec_path.display());

    let spec_content = fs::read_to_string(&spec_path)
        .context(format!("Erreur lecture spec {}", spec_path.display()))?;
    let spec: Value = serde_json::from_str(&spec_content).context("Erreur parse JSON")?;

    println!("  🔨 {} - Génération types...", config.name);
    let types_code = generate_types(&spec)?;
    let types_path = output_dir.join(config.types_file);
    fs::write(&types_path, &types_code).context(format!("Erreur écriture {}", config.types_file))?;
    println!("    ✅ Types : {}", types_path.display());

    println!("  🔨 {} - Génération client...", config.name);
    let client_code = generate_client(&spec)?;
    let client_path = output_dir.join(config.client_file);
    fs::write(&client_path, &client_code)
        .context(format!("Erreur écriture {}", config.client_file))?;
    println!("    ✅ Client : {}", client_path.display());

    if let Some(tools_file) = config.tools_file {
        println!("  🔨 {} - Génération MCP tools...", config.name);
        let tools_code = generate_mcp_tools(&spec)?;
        let tools_path = PathBuf::from(tools_file);
        if let Some(parent) = tools_path.parent() {
            fs::create_dir_all(parent).ok();
        }
        fs::write(&tools_path, &tools_code)
            .context(format!("Erreur écriture {}", tools_file))?;
        println!("    ✅ MCP tools : {}", tools_path.display());
    }

    Ok(())
}

fn update_mod_file(output_dir: &PathBuf) -> Result<()> {
    let mod_content = "// ⚠️  AUTO-GENERATED - NE PAS MODIFIER MANUELLEMENT\n// Regenerate: ./scripts/generate-client.sh\n\npub mod client;\npub mod mcp_tools;\npub mod types;\n\npub use client::ApiClient;\npub use mcp_tools::get_tool_definitions as get_mcp_tools;\npub use types::*;\n";
    fs::write(output_dir.join("mod.rs"), mod_content)?;
    println!("📝 mod.rs mise à jour");
    Ok(())
}

// ─── Type Generation ─────────────────────────────────────────────────────────

fn find_referenced_types(schema: &Value, mut refs: HashSet<String>) -> HashSet<String> {
    if let Some(ref_str) = schema.get("$ref").and_then(|r| r.as_str()) {
        if let Some(type_name) = ref_str.split('/').last() {
            refs.insert(type_name.to_string());
        }
    }
    if let Some(props) = schema.get("properties").and_then(|p| p.as_object()) {
        for (_, prop_schema) in props.iter() {
            refs = find_referenced_types(prop_schema, refs);
        }
    }
    if let Some(items) = schema.get("items") {
        refs = find_referenced_types(items, refs);
    }
    refs
}

fn detect_circular_references(schemas: &serde_json::Map<String, Value>) -> HashSet<String> {
    let mut circular = HashSet::new();
    for (name, schema) in schemas.iter() {
        let mut refs = find_referenced_types(schema, HashSet::new());
        refs.remove(name);
        for ref_name in refs.iter() {
            if let Some(ref_schema) = schemas.get(ref_name) {
                let back_refs = find_referenced_types(ref_schema, HashSet::new());
                if back_refs.contains(name) {
                    circular.insert(name.clone());
                    circular.insert(ref_name.clone());
                }
            }
        }
    }
    circular
}

fn generate_types(spec: &Value) -> Result<String> {
    let mut output = String::new();
    output.push_str("// ⚠️  AUTO-GENERATED - NE PAS MODIFIER MANUELLEMENT\n");
    output.push_str("// Generated from OpenAPI spec\n");
    output.push_str("// Regenerate: ./scripts/generate-client.sh\n\n");
    output.push_str("use serde::{Serialize, Deserialize};\n\n");

    if let Some(schemas) = spec
        .get("components")
        .and_then(|c| c.get("schemas"))
        .and_then(|s| s.as_object())
    {
        let circular = detect_circular_references(schemas);
        let mut schema_names: Vec<_> = schemas.keys().collect();
        schema_names.sort();

        for name in schema_names {
            if let Some(schema) = schemas.get(name) {
                if let Some(values) = string_enum_values(schema) {
                    output.push_str(&generate_enum(name, &values));
                } else {
                    output.push_str(&generate_struct(name, schema, &circular)?);
                }
                output.push('\n');
            }
        }
    }

    Ok(output)
}

fn string_enum_values(schema: &Value) -> Option<Vec<String>> {
    if schema.get("type").and_then(|t| t.as_str()) != Some("string") {
        return None;
    }
    let values = schema.get("enum")?.as_array()?;
    let values: Vec<String> = values.iter().filter_map(|v| v.as_str().map(str::to_string)).collect();
    if values.is_empty() {
        None
    } else {
        Some(values)
    }
}

fn to_pascal_case(snake: &str) -> String {
    snake
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

fn generate_enum(name: &str, values: &[String]) -> String {
    let mut output = String::new();
    output.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]\n");
    output.push_str("#[serde(rename_all = \"snake_case\")]\n");
    output.push_str(&format!("pub enum {} {{\n", name));
    for value in values {
        output.push_str(&format!("    {},\n", to_pascal_case(value)));
    }
    output.push_str("}\n");
    output
}

fn generate_struct(name: &str, schema: &Value, circular: &HashSet<String>) -> Result<String> {
    let mut output = String::new();

    let required: Vec<&str> = schema
        .get("required")
        .and_then(|r| r.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
        .unwrap_or_default();

    let has_required = !required.is_empty();

    if has_required {
        output.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
    } else {
        output.push_str("#[derive(Debug, Clone, Serialize, Deserialize, Default)]\n");
    }
    output.push_str("pub struct ");
    output.push_str(name);
    output.push_str(" {\n");

    if let Some(properties) = schema.get("properties").and_then(|p| p.as_object()) {
        let mut field_names: Vec<_> = properties.keys().collect();
        field_names.sort();

        for field_name in field_names {
            if let Some(field_schema) = properties.get(field_name) {
                let is_required = required.contains(&field_name.as_str());
                let rust_type = openapi_type_to_rust(field_schema, is_required, circular)?;
                let snake_name = to_snake_case(field_name);
                let needs_rename = snake_name != field_name.as_str() || is_keyword(&snake_name);

                if rust_type.starts_with("Option<") {
                    output.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                }

                output.push_str("    ");
                if needs_rename {
                    output.push_str(&format!("#[serde(rename = \"{}\")]\n    ", field_name));
                }
                output.push_str(&format!(
                    "pub {}: {},\n",
                    if is_keyword(&snake_name) {
                        format!("r#{}", snake_name)
                    } else {
                        snake_name
                    },
                    rust_type
                ));
            }
        }
    }

    output.push_str("}\n");
    Ok(output)
}

fn is_keyword(name: &str) -> bool {
    RUST_KEYWORDS.contains(&name)
}

fn openapi_type_to_rust(
    schema: &Value,
    is_required: bool,
    circular: &HashSet<String>,
) -> Result<String> {
    // Handle anyOf: [{type: T}, {type: null}] — OpenAPI 3.1 nullable pattern
    if let Some(any_of) = schema.get("anyOf").and_then(|a| a.as_array()) {
        let non_null: Vec<_> = any_of
            .iter()
            .filter(|v| v.get("type").and_then(|t| t.as_str()) != Some("null"))
            .collect();
        if non_null.len() == 1 {
            let inner = openapi_type_to_rust(non_null[0], true, circular)?;
            return Ok(format!("Option<{}>", inner));
        }
        return Ok("Option<serde_json::Value>".to_string());
    }

    let base_type = if let Some(ref_str) = schema.get("$ref").and_then(|r| r.as_str()) {
        let type_name = ref_str.split('/').last().unwrap_or("serde_json::Value").to_string();
        if circular.contains(&type_name) {
            "serde_json::Value".to_string()
        } else {
            type_name
        }
    } else if let Some(type_str) = schema.get("type").and_then(|t| t.as_str()) {
        match type_str {
            "string" => "String".to_string(),
            "integer" => "i64".to_string(),
            "number" => "f64".to_string(),
            "boolean" => "bool".to_string(),
            "array" => {
                if let Some(items) = schema.get("items") {
                    let item_type = openapi_type_to_rust(items, true, circular)?;
                    format!("Vec<{}>", item_type)
                } else {
                    "Vec<serde_json::Value>".to_string()
                }
            }
            "object" => "serde_json::Value".to_string(),
            _ => "serde_json::Value".to_string(),
        }
    } else {
        "serde_json::Value".to_string()
    };

    if is_required {
        Ok(base_type)
    } else {
        Ok(format!("Option<{}>", base_type))
    }
}

// ─── Client Generation ───────────────────────────────────────────────────────

fn generate_client(spec: &Value) -> Result<String> {
    let mut output = String::new();

    output.push_str("// ⚠️  AUTO-GENERATED - NE PAS MODIFIER MANUELLEMENT\n");
    output.push_str("// Generated from OpenAPI spec\n");
    output.push_str("// Regenerate: ./scripts/generate-client.sh\n\n");
    output.push_str("use reqwest::Client as HttpClient;\n");
    output.push_str("use anyhow::{Result, anyhow};\n");
    output.push_str("use super::types::*;\n\n");

    let mut type_map: HashMap<String, String> = HashMap::new();
    if let Some(schemas) = spec
        .get("components")
        .and_then(|c| c.get("schemas"))
        .and_then(|s| s.as_object())
    {
        for name in schemas.keys() {
            type_map.insert(name.clone(), name.clone());
        }
    }

    output.push_str("pub struct ApiClient {\n");
    output.push_str("    base_url: String,\n");
    output.push_str("    client: HttpClient,\n");
    output.push_str("}\n\n");

    output.push_str("impl ApiClient {\n");
    output.push_str("    pub fn new(base_url: impl Into<String>) -> Self {\n");
    output.push_str("        Self {\n");
    output.push_str("            base_url: base_url.into(),\n");
    output.push_str("            client: HttpClient::new(),\n");
    output.push_str("        }\n");
    output.push_str("    }\n\n");

    output.push_str("    async fn check_response(response: reqwest::Response) -> Result<reqwest::Response> {\n");
    output.push_str("        if !response.status().is_success() {\n");
    output.push_str("            let status = response.status();\n");
    output.push_str("            let body = response.text().await.unwrap_or_default();\n");
    output.push_str("            return Err(anyhow!(\"HTTP {}: {}\", status, body));\n");
    output.push_str("        }\n");
    output.push_str("        Ok(response)\n");
    output.push_str("    }\n\n");

    if let Some(paths) = spec.get("paths").and_then(|p| p.as_object()) {
        for (path, methods) in paths.iter() {
            if let Some(obj) = methods.as_object() {
                for (method, details) in obj.iter() {
                    if !["get", "post", "put", "delete", "patch"].contains(&method.as_str()) {
                        continue;
                    }
                    let Some(op_id) = details.get("operationId").and_then(|o| o.as_str()) else {
                        continue;
                    };

                    // Use summary as method name when available (cleaner than operationId)
                    let raw_name = details
                        .get("summary")
                        .and_then(|s| s.as_str())
                        .unwrap_or(op_id);
                    let method_name = to_snake_case(raw_name).replace(' ', "_");
                    let http_method = method.as_str();

                    let path_params = extract_path_params(path);
                    let query_params = extract_query_params(details);
                    let body_type = extract_body_type(details);
                    let return_type = extract_return_type_str(details, &type_map);

                    let mut sig_parts: Vec<String> = Vec::new();
                    for param in &path_params {
                        sig_parts.push(format!("{}: &str", to_rust_name(param)));
                    }
                    for (qname, qtype, _required) in &query_params {
                        sig_parts.push(format!("{}: Option<{}>", to_rust_name(qname), qtype));
                    }
                    if let Some(ref bt) = body_type {
                        sig_parts.push(format!("body: &{}", bt));
                    }

                    let params_str = if sig_parts.is_empty() {
                        String::new()
                    } else {
                        format!(", {}", sig_parts.join(", "))
                    };

                    output.push_str(&format!(
                        "    pub async fn {}(&self{}) -> Result<{}> {{\n",
                        method_name, params_str, return_type
                    ));

                    let url_path = path_params.iter().fold(path.clone(), |acc, p| {
                        acc.replace(&format!("{{{}}}", p), &format!("{{{}}}", to_rust_name(p)))
                    });
                    output.push_str(&format!(
                        "        let url = format!(\"{{base_url}}{}\", base_url = self.base_url{});\n",
                        url_path,
                        path_params.iter().map(|p| format!(", {} = {}", to_rust_name(p), to_rust_name(p))).collect::<Vec<_>>().join("")
                    ));

                    output.push_str(&format!(
                        "        let req = self.client.{}(&url);\n",
                        http_method
                    ));

                    if !query_params.is_empty() {
                        for (n, _, _) in &query_params {
                            let rn = to_rust_name(n);
                            output.push_str(&format!(
                                "        let req = if let Some(v) = {} {{ req.query(&[(\"{}\", v.to_string())]) }} else {{ req }};\n",
                                rn, n
                            ));
                        }
                    }

                    if body_type.is_some() {
                        output.push_str("        let req = req.json(body);\n");
                    }

                    output.push_str("        let response = Self::check_response(req.send().await?).await?;\n");

                    if return_type == "()" {
                        output.push_str("        Ok(())\n");
                    } else {
                        output.push_str("        Ok(response.json().await?)\n");
                    }

                    output.push_str("    }\n\n");
                }
            }
        }
    }

    output.push_str("}\n");
    Ok(output)
}

fn extract_path_params(path: &str) -> Vec<String> {
    let mut params = Vec::new();
    let mut in_p = false;
    let mut cur = String::new();
    for c in path.chars() {
        if c == '{' {
            in_p = true;
            cur.clear();
        } else if c == '}' && in_p {
            params.push(cur.clone());
            in_p = false;
        } else if in_p {
            cur.push(c);
        }
    }
    params
}

fn extract_query_params(details: &Value) -> Vec<(String, String, bool)> {
    let mut params = Vec::new();
    if let Some(ps) = details.get("parameters").and_then(|p| p.as_array()) {
        for p in ps {
            let location = p.get("in").and_then(|v| v.as_str()).unwrap_or("");
            if location != "query" {
                continue;
            }
            let name = match p.get("name").and_then(|n| n.as_str()) {
                Some(n) => n.to_string(),
                None => continue,
            };
            let required = p.get("required").and_then(|r| r.as_bool()).unwrap_or(false);
            let schema = p.get("schema").unwrap_or(&Value::Null);
            let rust_type = match schema.get("type").and_then(|t| t.as_str()) {
                Some("integer") => "i64",
                Some("number") => "f64",
                Some("boolean") => "bool",
                _ => "String",
            };
            params.push((name, rust_type.to_string(), required));
        }
    }
    params
}

fn extract_body_type(details: &Value) -> Option<String> {
    details
        .get("requestBody")
        .and_then(|rb| rb.get("content"))
        .and_then(|c| c.get("application/json"))
        .and_then(|j| j.get("schema"))
        .and_then(|s| s.get("$ref"))
        .and_then(|r| r.as_str())
        .and_then(|r| r.split('/').last())
        .map(|s| s.to_string())
}

fn extract_return_type_str(details: &Value, type_map: &HashMap<String, String>) -> String {
    if let Some(responses) = details.get("responses").and_then(|r| r.as_object()) {
        if responses.contains_key("204") && !responses.contains_key("200") && !responses.contains_key("201") {
            return "()".to_string();
        }
        for status_code in &["200", "201"] {
            if let Some(response) = responses.get(*status_code) {
                if let Some(schema) = response
                    .get("content")
                    .and_then(|c| c.get("application/json"))
                    .and_then(|j| j.get("schema"))
                {
                    if let Some(ref_str) = schema.get("$ref").and_then(|r| r.as_str()) {
                        if let Some(type_name) = ref_str.split('/').last() {
                            if type_map.contains_key(type_name) {
                                return type_name.to_string();
                            }
                        }
                    }
                    if schema.get("type").and_then(|t| t.as_str()) == Some("array") {
                        if let Some(items) = schema.get("items") {
                            if let Some(ref_str) = items.get("$ref").and_then(|r| r.as_str()) {
                                if let Some(type_name) = ref_str.split('/').last() {
                                    if type_map.contains_key(type_name) {
                                        return format!("Vec<{}>", type_name);
                                    }
                                }
                            }
                        }
                        return "Vec<serde_json::Value>".to_string();
                    }
                }
            }
        }
    }
    "serde_json::Value".to_string()
}

// ─── MCP Tools Generation ────────────────────────────────────────────────────

fn generate_mcp_tools(spec: &Value) -> Result<String> {
    let mut output = String::new();
    output.push_str("// ⚠️  AUTO-GENERATED - NE PAS MODIFIER MANUELLEMENT\n");
    output.push_str("// Generated from OpenAPI spec\n");
    output.push_str("// Regenerate: ./scripts/generate-client.sh\n\n");
    output.push_str("use serde_json::{json, Value};\n\n");
    output.push_str("/// Returns all MCP tool definitions for the Annuaire API.\n");
    output.push_str("pub fn get_tool_definitions() -> Vec<Value> {\n");
    output.push_str("    vec![\n");

    if let Some(paths) = spec.get("paths").and_then(|p| p.as_object()) {
        for (path, methods) in paths.iter() {
            if let Some(obj) = methods.as_object() {
                for (method, details) in obj.iter() {
                    if !["get", "post", "put", "delete", "patch"].contains(&method.as_str()) {
                        continue;
                    }
                    let Some(op_id) = details.get("operationId").and_then(|o| o.as_str()) else {
                        continue;
                    };
                    let summary = details.get("summary").and_then(|s| s.as_str()).unwrap_or(op_id);
                    let description = details
                        .get("description")
                        .and_then(|d| d.as_str())
                        .unwrap_or(summary);
                    let tool_name = to_rust_name(op_id);

                    output.push_str(&format!("        json!({{\n"));
                    output.push_str(&format!("            \"name\": \"{}\",\n", tool_name));
                    output.push_str(&format!("            \"description\": {:?},\n", description));
                    output.push_str("            \"inputSchema\": {\n");
                    output.push_str("                \"type\": \"object\",\n");
                    output.push_str("                \"properties\": {}\n");
                    output.push_str("            }\n");
                    output.push_str("        }),\n");
                    let _ = (path, method);
                }
            }
        }
    }

    output.push_str("    ]\n");
    output.push_str("}\n");
    Ok(output)
}

// ─── Utilities ───────────────────────────────────────────────────────────────

fn to_snake_case(camel: &str) -> String {
    let chars: Vec<char> = camel.chars().collect();
    chars
        .iter()
        .enumerate()
        .fold(String::new(), |mut acc, (i, &c)| {
            if c.is_whitespace() || c == '_' || c == '-' {
                if !acc.is_empty() && !acc.ends_with('_') {
                    acc.push('_');
                }
                return acc;
            }
            if i > 0 && c.is_uppercase() && !acc.ends_with('_') && !chars[i - 1].is_whitespace() {
                acc.push('_');
            }
            acc.push(c.to_lowercase().next().unwrap_or(c));
            acc
        })
        .trim_matches('_')
        .to_string()
}

fn to_rust_name(operation_id: &str) -> String {
    let snake = operation_id
        .chars()
        .enumerate()
        .fold(String::new(), |mut acc, (i, c)| {
            if i > 0 && c.is_uppercase() && !acc.ends_with('_') {
                acc.push('_');
            }
            acc.push(c.to_lowercase().next().unwrap_or(c));
            acc
        });
    let snake = snake.trim_matches('_').to_string();
    if is_keyword(&snake) {
        format!("r#{}", snake)
    } else {
        snake
    }
}
