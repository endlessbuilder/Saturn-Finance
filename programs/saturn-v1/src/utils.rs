use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Platform {
    id: u8,
    return_rate: f64,
    risk_rating: f64,
    allocation: f64,
    platform_type: u8,
}

pub fn re_allocate(treasur: &Vec<Platform>, platform_allocation: &Vec<f64>) -> Vec<Platform> {
    let mut platform_type_ratings: std::collections::HashMap<u8, f64> = std::collections::HashMap::new();
    let mut platform_type_allocations: std::collections::HashMap<u8, f64> = std::collections::HashMap::new();
    let mut platform_type_counts: std::collections::HashMap<u8, u8> = std::collections::HashMap::new();

    for platform in treasur.iter() {
        let platform_type = platform.platform_type;
        let return_rate = platform.return_rate;
        let risk_rating = platform.risk_rating;
        let allocation = platform.allocation;

        let rating = if risk_rating != 0.0 { return_rate / risk_rating } else { 0.0 };

        *platform_type_ratings.entry(platform_type).or_insert(0.0) += rating;
        *platform_type_allocations.entry(platform_type).or_insert(0.0) += allocation;
        *platform_type_counts.entry(platform_type).or_insert(0) += 1;
    }

    let mut new_allocations = Vec::new();

    for platform in treasur.iter() {
        let platform_type = platform.platform_type;
        let rating = if platform.risk_rating != 0.0 { platform.return_rate / platform.risk_rating } else { 0.0 };
        let total_rating = platform_type_ratings.get(&platform_type).unwrap_or(&0.0);
        let total_allocation = platform_allocation[(platform_type - 1) as usize];

        let new_allocation = if *total_rating != 0.0 { (rating / total_rating) * total_allocation } else { 0.0 };

        let new_platform = Platform {
            id: platform.id,
            return_rate: platform.return_rate,
            risk_rating: platform.risk_rating,
            allocation: new_allocation,
            platform_type: platform.platform_type,
        };

        new_allocations.push(new_platform);
    }

    new_allocations
}

/***** 
    fn main() {
        let marginfi = Platform { id: 1, return_rate: 52.0, risk_rating: 5.0, allocation: 15.0, platform_type: 1 };
        let kamino = Platform { id: 2, return_rate: 32.0, risk_rating: 7.0, allocation: 10.0, platform_type: 1 };
        let meteora = Platform { id: 3, return_rate: 72.0, risk_rating: 3.0, allocation: 17.5, platform_type: 2 };
        let jupiterperps = Platform { id: 4, return_rate: 152.0, risk_rating: 8.0, allocation: 10.0, platform_type: 3 };
        let usdcoin = Platform { id: 5, return_rate: 1.0, risk_rating: 1.0, allocation: 22.5, platform_type: 4 };
        let btc = Platform { id: 6, return_rate: 1.0, risk_rating: 2.0, allocation: 15.0, platform_type: 4 };
        let sol = Platform { id: 7, return_rate: 1.0, risk_rating: 4.0, allocation: 10.0, platform_type: 4 };

        let treasur = vec![marginfi, kamino, meteora, jupiterperps, usdcoin, btc, sol];

        let platform_allocation = vec![35.0, 25.0, 10.0, 30.0];

        let new_allocations = re_allocate(&treasur, &platform_allocation);

        for allocation in new_allocations.iter() {
            println!("Platform ID: {}, New Allocation: {}", allocation.id, allocation.allocation);
        }
    }
******/