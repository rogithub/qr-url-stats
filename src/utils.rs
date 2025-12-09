use url::Url;

pub fn validate_url(url_str: &str) -> Result<String, String> {
    // Intentar parsear el URL
    let parsed = Url::parse(url_str)
        .map_err(|_| "URL inválida".to_string())?;
    
    // Verificar que tenga esquema http o https
    if parsed.scheme() != "http" && parsed.scheme() != "https" {
        return Err("Solo se permiten URLs con http:// o https://".to_string());
    }
    
    // Verificar que tenga un host
    if parsed.host_str().is_none() {
        return Err("El URL debe tener un dominio válido".to_string());
    }
    
    // Retornar el URL normalizado (el crate url lo limpia automáticamente)
    Ok(parsed.to_string())
}