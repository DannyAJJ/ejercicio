use std::fs::OpenOptions;
use std::io::*;
use std::fs::File;
use std::collections::HashMap;

/*fn leer() -> std::io::Result<()>{
    let mut archivo=File::open("puntaje.txt")?;
    let mut contenido=String::new();
    archivo.read_to_string(&mut contenido)?;
    println!("El puntaje es: {}", contenido);
    Ok(())
}

fn agregar() -> std::io::Result<()>{
    const CONTENIDO:&str="\nprueba";
    let mut archivo = OpenOptions::new().append(true).create(true).open("Puntuacion.txt")?;
    archivo.write_all(CONTENIDO.as_bytes())?;
    Ok(())
}*/
fn puntajet(x : Vec<f64>, h : (f64, f64)){
    let mut puntajet : f64 = 0.00;
    puntajet = x.iter().sum();
    let mut puntajef : f64 = (puntajet - h.0 - h.1) / 3.00;
    println!("El puntaje final del Participante es: {}", puntajef);
}
fn mayormenor(x : Vec<f64>) -> (f64, f64){
    let mut tmamen : (f64,f64) = (0.00, 0.00);
    let mut mayor: f64 = std::f64::NEG_INFINITY;
    let mut menor:f64 = std::f64::INFINITY;
    for h in x{
        if h > mayor{
            mayor = h;
        }
        if h < menor{
            menor = h;
        }
    }
    tmamen = (mayor, menor);
    println!("Este es el puntaje mas alto: {}, Este es el puntaje mas bajo: {}", tmamen.0, tmamen.1);
    tmamen
}

fn puntajeb(pun : Vec<(f64, f64)>, h : (f64, f64)) -> Vec<f64>{
    let mut puntajesf : Vec<f64> = Vec::new();
    let mut puntajef : f64 = 0.00;
    for h in pun{
        let (pa,pt) = h;
        puntajef = ((((((pa*40.00).round())/100.00)+(((pt*60.00).round())/100.00))*100.00).round())/100.00;
        puntajesf.push(puntajef);
    }
    puntajesf
}
fn main() {
    let nvector : Vec<f64> = Vec::new();
    let mut diccionario: HashMap<String, Vec<f64>> = HashMap::new();
    let mut par:i64 = 1;
    let mut vector : Vec<(f64,f64)> = Vec::new();
    println!("¿Cuantos participantes hay?");
    let mut participantes: String = String::new();
    std::io::stdin().read_line(&mut participantes).unwrap();
    let mut nparticipantes: i64 = participantes.trim().parse().unwrap();
    println!("¿Cuantos Jueces Hay?");
    let mut jueces = String::new();
    std::io::stdin().read_line(&mut jueces).unwrap();
    let mut njueces:i64 = jueces.trim().parse().unwrap();
    let mut nombre: String = String::new();
    let mut e: (f64, f64) = (0.00, 0.00);
    while par<=nparticipantes{
        println!("Ingrese el nombre del participante N°{}", par);
        std::io::stdin().read_line(&mut nombre).unwrap();
        let mut jue:i64 = 1;
        while jue<=njueces {
            println!("Juez Numero N° {}, por favor ingrese sus calificaciones", jue);
            println!("PUNTAJE ARTISTICO:");
            let mut puntoa: String = String::new();
            std::io::stdin().read_line(&mut puntoa).unwrap();
            let puntosa:f64 = puntoa.trim().parse().unwrap();
            println!("PUNTAJE TECNICO:");
            let mut puntot: String = String::new();
            std::io::stdin().read_line(&mut puntot).unwrap();
            let puntost:f64 = puntot.trim().parse().unwrap();
            e = (puntosa,puntost);
            vector.push(e);
            jue=jue+1;
        }
        par=par+1;
    }
    let nvector = puntajeb(vector, e);
    let mayores = mayormenor(nvector);
    let mut nvector1 : Vec<f64> = puntajeb(vector, e);
    puntajet(nvector1, mayores);
}