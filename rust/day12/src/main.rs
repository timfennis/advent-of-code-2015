use anyhow::Context;
use serde_json::Value;

fn main() -> anyhow::Result<()> {
    let v: Value = serde_json::from_str(include_str!("../input"))?;

    println!("Part 1: {}", sum_numbers(&v, false)?);
    println!("Part 1: {}", sum_numbers(&v, true)?);

    Ok(())
}

fn sum_numbers(node: &Value, ignore_red: bool) -> anyhow::Result<i64> {
    match node {
        Value::Array(arr) => arr.iter().map(|v| sum_numbers(v, ignore_red)).sum(),
        Value::Object(map) => {
            if ignore_red
                && map.iter().any(|(_, v)| match v {
                    Value::String(s) if s == "red" => true,
                    _ => false,
                })
            {
                return Ok(0);
            }

            map.iter().map(|(_, v)| sum_numbers(v, ignore_red)).sum()
        }
        Value::Number(number) => number.as_i64().context("unable to convert Number to i64"),
        _ => Ok(0),
    }
}
