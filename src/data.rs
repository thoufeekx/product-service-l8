use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Apple iPhone 15".to_string(),
            price: 999.99,
            description: "Experience the power and precision of the iPhone 15. With an advanced A16 chip, 5G connectivity, and a stunning OLED display, it's the perfect combination of performance and design.".to_string(),
            image: "/iphone15.jpg".to_string(),
        },
        Product {
            id: 2,
            name: "Dell XPS 13 Laptop".to_string(),
            price: 1099.99,
            description: "The Dell XPS 13 is the ultimate laptop for work and play. Featuring a sleek design, powerful Intel Core i7 processor, and a 13.3-inch 4K display for vibrant visuals.".to_string(),
            image: "/dell_xps13.jpg".to_string(),
        },
        Product {
            id: 3,
            name: "Apple MacBook Pro 14-inch".to_string(),
            price: 1799.99,
            description: "The MacBook Pro 14-inch is built for professional users with the latest Apple M1 Pro chip, offering unprecedented speed and graphics performance for heavy-duty tasks.".to_string(),
            image: "/macbookpro14.jpg".to_string(),
        },
        Product {
            id: 4,
            name: "Logitech MX Master 3 Mouse".to_string(),
            price: 99.99,
            description: "Maximize your productivity with the Logitech MX Master 3. A wireless mouse designed for precision, comfort, and functionality with customizable buttons and an ergonomic design.".to_string(),
            image: "/logitech_mx_master3.jpg".to_string(),
        },
        Product {
            id: 5,
            name: "Sony WH-1000XM5 Wireless Headphones".to_string(),
            price: 349.99,
            description: "Immerse yourself in crystal-clear audio with the Sony WH-1000XM5. These noise-canceling headphones feature superior sound quality, long-lasting battery life, and a comfortable fit.".to_string(),
            image: "/sony_wh1000xm5.jpg".to_string(),
        },
        Product {
            id: 6,
            name: "Samsung Galaxy S23 Ultra".to_string(),
            price: 1199.99,
            description: "The Samsung Galaxy S23 Ultra offers a premium mobile experience with its 200 MP camera, powerful Snapdragon 8 Gen 2 processor, and a stunning 6.8-inch Dynamic AMOLED 2X display.".to_string(),
            image: "/galaxy_s23_ultra.jpg".to_string(),
        },
        Product {
            id: 7,
            name: "Razer BlackWidow V3 Mechanical Keyboard".to_string(),
            price: 129.99,
            description: "The Razer BlackWidow V3 is a mechanical keyboard designed for gamers, offering tactile key switches, customizable RGB lighting, and advanced anti-ghosting technology.".to_string(),
            image: "/razer_blackwidow_v3.jpg".to_string(),
        },
        Product {
            id: 8,
            name: "Sony PlayStation 5 Console".to_string(),
            price: 499.99,
            description: "PlayStation 5 delivers an unparalleled gaming experience with lightning-fast load times, immersive 3D audio, and ultra-high-definition visuals.".to_string(),
            image: "/ps5_console.jpg".to_string(),
        },
        Product {
            id: 9,
            name: "Samsung 32-inch 4K Monitor".to_string(),
            price: 349.99,
            description: "Upgrade your setup with the Samsung 32-inch 4K monitor, featuring vibrant colors, ultra-clear resolution, and a wide viewing angle for an immersive viewing experience.".to_string(),
            image: "/samsung_32inch_4k_monitor.jpg".to_string(),
        },
        Product {
            id: 10,
            name: "Bose SoundLink Revolve+ Bluetooth Speaker".to_string(),
            price: 329.99,
            description: "Enjoy deep, immersive sound with the Bose SoundLink Revolve+ Bluetooth speaker. This portable speaker delivers 360-degree audio and is water-resistant, perfect for any occasion.".to_string(),
            image: "/bose_soundlink_revolve.jpg".to_string(),
        }
    ]
}
