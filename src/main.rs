use ibex::{routes, ssg};

mod posts;
mod routes;
mod rss;
mod views;

const URL_ROOT: &str = "/garfeo2/";

fn main() {
    let posts = posts::parse_posts().expect("Failed to parse posts");

    let routes = routes! [
        // Normal pages
        (/)
            => routes::at_index(&posts),
        (/"plej-bonaj")
            => routes::at_favorites(&posts),
        (/"informejo")
            => routes::at_about(&posts),
        (/"instrukcio")
            => routes::at_instructions(&posts),
        (/"listo")
            => routes::at_list(&posts),
        (/404)
            => routes::at_404(&posts),

        // Posts (HTML)
        (/[post.get().index()])
            for post in posts
            => routes::at_post(post),
        (/"lasta")
            => routes::at_post(posts.first_ref()),

        // Posts (JSON)
        (/"index.json") | (/"posts.json")
            => ssg::raw(routes::json_index(&posts)),
        (/[post.get().index()]".json")
            for post in posts
            => ssg::raw(routes::json_post(post)),
        (/"latest.json")
            => ssg::raw(routes::json_post(posts.first_ref())),

        // RSS file
        (/"rss.xml")
            => ssg::raw(rss::generate_rss(posts)),
    ];

    ssg::quick_build(routes).expect("Failed to build");
    println!("\x1b[34;1mBuilt successfully!\x1b[0m");
}
