use core::f64::consts::PI;
use rand::Rng;
use csv::{ReaderBuilder, Writer, Trim,};
use std::io::Read;
use std::fs::File;
use std::io::Write;

use std::error::Error;


#[derive(Debug, Clone, Copy)]
struct Circle {
    x:f64,
    y:f64,
    r:f64,
}
#[derive(Debug, Clone)]
struct Layer {
    name: String,
    color: String,
    list_circle: Vec<Circle>,
}

fn main() {
    csv_to_html2()
}

fn gen_obj_layer_list<R: Rng>(rng: &mut R, n:i32) -> Vec<Layer> {
    let mut result = Vec::new();
    

    for i in 1..n + 1 {

        let mut point = Vec::new();
        let ni = rng.gen_range(20 ..= 50);
        for _ in 0..ni {
            let x = rng.gen_range(-100. ..= 100.);
            let y = rng.gen_range(-100. ..= 100.);
            let r = rng.gen_range(-10. ..= 20.);

            point.push(Circle{x, y, r})
        }

        let r : u8= rng.gen();
        let g : u8= rng.gen();
        let b : u8= rng.gen();
        let color = format!("#{:02X}{:02X}{:02X}", r, g, b);
        let name = format!("Layer {}", i);
        result.push(Layer { name, color, list_circle: point.clone()})
    }
    
    result
}

#[test]
fn test_gen_layer_list() {
    let mut rng = rand::thread_rng();
    let n = 5;
    let layers = gen_obj_layer_list(&mut rng, n);

    for (i, layer) in layers.iter().enumerate() {

        for point in &layer.list_circle {
            assert!(point.x >= -100.0 && point.x <= 100.0);
            assert!(point.y >= -100.0 && point.y <= 100.0);
            assert!(point.r >= -10.0 && point.r <= 20.0)
        }

        assert_eq!(layer.name, format!("Layer {}", i + 1))
    }
}

fn cal_average_area(layers: Vec<Layer>) -> Vec<(Layer, f64)> {
    let mut result = Vec::new();
    for layer in layers {
        let mut sum = 0.;
        for circle in &layer.list_circle {
            let area = PI as f64 * circle.r.powi(2);
            sum += area
        }
        let avg = sum / layer.list_circle.len() as f64;
        result.push((layer.clone(), avg))
    }

    result
}

fn cal_max_min(layers: Vec<Layer>) -> Vec<(f64, f64)> {
    let mut result = Vec::new();

    for layer in layers {
        let mut list = Vec::new();

        for circle in &layer.list_circle {
            let area = std::f64::consts::PI * circle.r.powi(2);
            list.push(area);
        }

        let max = list.iter().copied().fold(None, |acc, x| {
            match acc {
                None => Some(x),
                Some(max_val) => Some(f64::max(max_val, x)),
            }
        }).unwrap_or(0.0);

        let min = list.iter().copied().fold(None, |acc, x| {
            match acc {
                None => Some(x),
                Some(min_val) => Some(f64::min(min_val, x)),
            }
        }).unwrap_or(0.0);

        result.push((max, min));
    }

    result
}




#[test]
fn test_cal_average_area() {
    // Create some sample layers with circles
    let layer1 = Layer {
        name: "Layer 1".to_string(),
        color: "#FF0000".to_string(),
        list_circle: vec![
            Circle { x: 0.0, y: 0.0, r: 5.0 },
            Circle { x: 0.0, y: 0.0, r: 10.0 },
        ],
    };

    let layer2 = Layer {
        name: "Layer 2".to_string(),
        color: "#00FF00".to_string(),
        list_circle: vec![
            Circle { x: 0.0, y: 0.0, r: 7.0 },
            Circle { x: 0.0, y: 0.0, r: 14.0 },
        ],
    };

    
    let layers = vec![layer1, layer2];
    let result = cal_average_area(layers);

    assert_eq!(result.len(), 2);

    assert_eq!(result[0].0.name, "Layer 1");
    assert_eq!(result[0].1, ((PI as f64 * 5.0 * 5.0) + (PI as f64 * 10.0 * 10.0)) / 2.0);

    assert_eq!(result[1].0.name, "Layer 2");
    assert_eq!(result[1].1, ((PI as f64 * 7.0 * 7.0) + (PI as f64 * 14.0 * 14.0 ))/ 2.0);
}

fn write_layer<W: Write>(writer: W, layers: Vec<Layer>) {
    let mut wtr = csv::Writer::from_writer(writer);

    for layer in layers {
        let mut records = Vec::new();

        let points_str = layer
            .list_circle
            .iter()
            .map(|point| format!("{},{},{}", point.x, point.y, point.r))
            .collect::<Vec<String>>()
            .join(";");

        records.push(layer.name);
        records.push(layer.color);
        records.push(points_str);

        wtr.write_record(&records).unwrap();
    }

    wtr.flush().unwrap();
}

fn to_csv() {
    let mut rng= rand::thread_rng();
    let n = rng.gen_range(20..=50);
    let layers = gen_obj_layer_list(&mut rng, n);
    let result = write_layer(File::create("input.csv").unwrap(), layers);
    
    result

}

//2.2
fn read_layers_from_csv<R: Read>(rdr: R) -> Vec<Layer> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .trim(Trim::All)
        .from_reader(rdr);
    let mut layers = vec![];
    
    for record in reader.records() {
        if let Ok(rec) = record {
            let name = rec[0].to_string();
            let color = rec[1].to_string();
            
            let circles_data: Vec<&str> = rec[2].split(';').map(|s| s.trim()).collect();
            let mut circles = vec![];
            
            for circle_str in circles_data {
                let parts: Vec<&str> = circle_str.split(',').collect();
                
                if parts.len() == 3 {
                    let x: f64 = parts[0].parse().unwrap();
                    let y: f64 = parts[1].parse().unwrap();
                    let r: f64 = parts[2].parse().unwrap();
                    
                    circles.push(Circle { x, y, r });
                }
            }
            
            layers.push(Layer { name, color, list_circle: circles });
        }
    }
    layers
}

fn save_result_to_csv(file_path: &str, result: &[(Layer, f64)]) -> Result<(), Box<dyn Error>> {
    let file = File::create(file_path)?;
    let mut wtr = Writer::from_writer(file);

    for (layer, avg) in result {
        // Format the average as a string
        let avg_str = format!("{:.2}", avg); 

        wtr.write_record(&[layer.name.clone(), layer.color.clone(), avg_str])?;

        
    }

    wtr.flush()?;
    Ok(())
}


fn csv_to_csv() {
     

      let file = File::open("input.csv").unwrap();
      let layers = read_layers_from_csv(file);

      let result = cal_average_area(layers);
  
     
      save_result_to_csv("output2.csv", &result).unwrap();
}

fn csv_to_html() {
  
    let file = File::open("input.csv").unwrap();
    let layers = read_layers_from_csv(file);

    let result = cal_average_area(layers);

    let mut table = String::new();
    table.push_str("<!DOCTYPE html>
    <html>
        <head>
            <title>Average circle value</title>
            <style> table, th, td {
                border: 1px solid #000000;
                text-align: center;
                width: 50%;
                border-collapse: collapse; 
                }
            </style>
            <h1>Average circle value</h1>
        </head>
        <body>
            <table>
                <thead>
                    <tr>
                        <th>layer</th>
                        <th>color</th>
                        <th>average</th>
                    </tr>
                </thead>
                <tbody>"
    );
    for (layer, avg) in result{
        table.push_str("<tr>");
        table.push_str(&format!("<td>{}</td>", layer.name));
        table.push_str(&format!("<td>{}</td>", layer.color));
        table.push_str(&format!("<td>{:.2}</td>", avg));
        table.push_str("</tr>");
    }
    table.push_str("</tbody></table></body></html>");
    println!("{}", table)
    
    
}

fn csv_to_html2() {
   
    let file = File::open("input.csv").unwrap();
    let layers = read_layers_from_csv(file);

    
    let result = cal_average_area(layers.clone());
    let max_min = cal_max_min(layers);

    let mut table = String::new();
    table.push_str("<!DOCTYPE html>
    <html>
        <head>
            <title>Average circle value</title>
            <style> table, th, td {
                border: 1px solid #000000;
                text-align: center;
                width: 50%;
                border-collapse: collapse; 
                }
            </style>
            <h1>Average circle value</h1>
        </head>
        <body>
            <table>
                <thead>
                    <tr>
                        <th>layer</th>
                        <th>color</th>
                        <th>average</th>
                        <th>max</th>
                        <th>min</th>
                    </tr>
                </thead>
                <tbody>"
    );
    for ((layer, avg), (max, min)) in result.iter().zip(max_min) {
        table.push_str("<tr>");
        table.push_str(&format!("<td>{}</td>", layer.name)); 
        table.push_str(&format!("<td>{}</td>", layer.color)); 
        table.push_str(&format!("<td>{:.2}</td>", avg));
        table.push_str(&format!("<td>{:?}</td>", max));
        table.push_str(&format!("<td>{:?}</td>", min));
        table.push_str("</tr>");
    }
    table.push_str("</tbody></table></body></html>");
    println!("{}", table)
    
    
}