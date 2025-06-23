use redis::{Commands, Connection, RedisResult};

pub fn guardar_resultado_count(redis: &mut Connection, archivo: &str, part: &str, valor: usize) -> RedisResult<()> {
    let clave = format!("count:{}:{}", archivo, part);
    redis.set(clave, valor)
}

pub fn obtener_resultados_count(redis: &mut Connection, archivo: &str) -> RedisResult<Vec<usize>> {
    let patron = format!("count:{}:*", archivo);
    let claves: Vec<String> = redis.keys(patron)?;
    let mut resultados = vec![];

    for clave in claves.iter() {
        if let Ok(valor_str) = redis.get::<_, String>(clave) {
            if let Ok(valor) = valor_str.parse::<usize>() {
                resultados.push(valor);
            }
        }
    }

    Ok(resultados)
}

pub fn borrar_resultados_count(redis: &mut Connection, archivo: &str) -> RedisResult<()> {
    let patron = format!("count:{}:*", archivo);
    let claves: Vec<String> = redis.keys(patron)?;

    for clave in claves.iter() {
        let _ = redis.del::<_, ()>(clave);
    }

    Ok(())
}
