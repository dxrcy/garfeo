pub struct BlogPost {
    pub title: String,
    pub author: String,
    pub body: String,
    pub image: String,
}

pub fn get_blog_posts() -> Vec<BlogPost> {
    vec![
        BlogPost {
            title: "Exploring the Beauty of Nature".to_string(),
            author: "Alice Johnson".to_string(),
            body: "Nature's wonders never cease to amaze us. From lush green forests to tranquil lakes, we dive into the breathtaking beauty of the great outdoors. Join us on this visual journey.".to_string(),
            image: "https://images.pexels.com/photos/2686558/pexels-photo-2686558.jpeg?auto=compress&cs=tinysrgb&w=600".to_string(),
        },
        BlogPost {
            title: "The Art of Cooking: A Culinary Adventure".to_string(),
            author: "John Smith".to_string(),
            body: "Discover the joy of cooking with these mouthwatering recipes. From gourmet dishes to homey comfort food, we explore the diverse world of culinary delights.".to_string(),
            image: "https://images.pexels.com/photos/3785693/pexels-photo-3785693.jpeg?auto=compress&cs=tinysrgb&w=600".to_string(),
        },
        BlogPost {
            title: "The Future of Space Exploration".to_string(),
            author: "Ella Davis".to_string(),
            body: "Humanity's quest to reach the stars continues. In this article, we delve into the latest developments in space exploration and the exciting possibilities that lie ahead.".to_string(),
            image: "https://images.pexels.com/photos/41951/solar-system-emergence-spitzer-telescope-telescope-41951.jpeg?auto=compress&cs=tinysrgb&w=600".to_string(),
        },
        BlogPost {
            title: "Tech Trends: Innovations Shaping Tomorrow".to_string(),
            author: "Michael Lee".to_string(),
            body: "Stay up to date with the latest tech trends that are revolutionizing our world. From AI and robotics to cybersecurity, we explore the technologies that are shaping the future.".to_string(),
            image: "https://images.pexels.com/photos/3862632/pexels-photo-3862632.jpeg?auto=compress&cs=tinysrgb&w=600".to_string(),
        },
    ]
}
