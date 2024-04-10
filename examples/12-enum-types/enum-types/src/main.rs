#[derive(Debug)]
enum WineRegions {
    Bordeaux,
    Burgundy,
    Champagne,
    Tuscany,
    Rioja,
    NapaValley,
    Mars,
}

struct Wine {
    name: String,
    region: WineRegions, // wine regions used as a type
}

fn supported_regions(w: WineRegions) {
    match w {
        WineRegions::Rioja => println!("Rioja is supported!"),
        _ => println!("{:?} is not supported!", w),
    }
}

fn wine_popularity(w: WineRegions) {
    match w {
        WineRegions::Bordeaux => println!("Highly Popular"),
        WineRegions::Mars => println!("Fictious"),
        WineRegions::Champagne => println!("Popolar"),
        _ => println!("Dunno"),
    }
}

fn main() {
    let wine1 = Wine {
        name: String::from("Chateau Margaux"),
        region: WineRegions::Bordeaux,
    };

    let wine2 = Wine {
        name: String::from("Barolo"),
        region: WineRegions::Tuscany,
    };

    let wine3 = Wine {
        name: String::from("Valles Marineris"),
        region: WineRegions::Mars,
    };

    // println!("Wine 1: {} from {:?}", wine1.name, wine1.region);
    // println!("Wine 2: {} from {:?}", wine2.name, wine2.region);
    supported_regions(wine1.region);
    supported_regions(WineRegions::Rioja);
    wine_popularity(WineRegions::NapaValley);
}
