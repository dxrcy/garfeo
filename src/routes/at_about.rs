use ibex::prelude::*;

use crate::posts::PostList;
use crate::views::use_base;

pub fn at_about(posts: &PostList) -> Document {
    document! { [lang="eo"] @use_base [
        "Informejo",
        None,
        posts,
    ] {
        h2 { "Oftaj Demandoj" }
        div ."instructions highlight-links" {
            h3 { "Kio estas Garfield-EO?" }
            p {
                "Mi tradukas bildstriojn de Garfildo en Esperanton."
                br/
                "Parto de"
                ~ a [href="https://instagram.com/mondodakomiksoj"] {
                    i{"Mondo da Komiksoj"}
                }
                "."
            }

            h3 { "Kiel vi trovas bildstriojn?" }
            p {
                "Garfildo-bildstrioj troviĝas je la retejo"~ a[href="https://gocomics.com/garfield"]{"GoComics.com"} "."
                ~
                "Oni povas elŝuti"~ i{"ĉiun"} ~"Garfildo-bildstrion per"~ a[href="https://github.com/dxrcy/everygarf"]{"EveryGarf"} "."
            }

            h3 { "Kiel vi redaktas bildstriojn?" }
            p {
                "Mi uzas la programon '" a[href="http://inkscape.org/"]{"InkScape"} "' por redakti bildojn."
                ~
                "Oni povas elŝutu la bildstria-tiparon"~ a[href=url!("static/GarfieldSans-EO-Regular.ttf")]{"ĉi tie"} "."
                ~
                "Mi organizas, formatas, kaj redaktas la bildstriojn per la skriptaro "~ a[href="https://github.com/dxrcy/garfutils"]{"garfutils"} "."
            }

            h3 { "Kiel vi kreas ĉi tiun retpaĝo?" }
            p {
                "Ĉi tiu retpaĝo estas kreiita per la ret-kadro"~ a[href="https://github.com/dxrcy/ibex"]{"Ibex"} ~"en la programlingvo"~ a[href="https://www.rust-lang.org/"]{"Rust"} "."
                ~
                "La fontkodo troviĝas"~ a[href="https://github.com/dxrcy/garfeo"]{"ĉi tie"} ","
                ~"kaj la fonto de ĉiu tradukita bildstrio troviĝas"~ a[href="https://github.com/dxrcy/garfeo/tree/master/assets/posts"]{"ĉi tie"} "."
            }
            h3 { "Mi havas concernon, aŭ alia demandon!" }
            p {
                "Informu min per GitHub"~ a[href="https://github.com/dxrcy/garfeo/issues/new"]{"ĉi tie"} "."
            }
        }

        h2 { "Ligiloj" }
        ul ."links" {
            li { a [href="https://github.com/dxrcy/garfeo"]
                { strong { "Fontkodo kaj ĉiu bildstrio" }
                    ~ "- por ĉi tiu retejo (en la angla)" }
            }
            li { a [href="https://github.com/dxrcy/garfeo/issues/new"]
                { strong { "Mi havas concernon!" }
                    ~ "- Informu min per GitHub" }
            }
            li { a [href="https://github.com/dxrcy/everygarf"]
                { strong { "EveryGarf" }
                    ~ "- Elŝuti ĉiujn Garfildajn bildstriojn ĝisnune" }
            }
            li { a [href="https://mastodon.world/@garfieldeo"]
                { strong { "Mastodon @garfieldeo@mastodon.world" }
                    ~ "- Esperantaj Garfildaj bildstrioj" }
            }
            li { a [href="https://instagram.com/garfield.eo"]
                { strong { "Instagram @garfield.eo" }
                    ~ "- Esperantaj Garfildaj bildstrioj" }
            }
            li { a [href="https://instagram.com/mondodakomiksoj"]
                { strong { "Mondo da Komiksoj" }
                    ~ "- Grupo de tradukistoj" }
            }
            li { a [href=url!("rss.xml")]
                { strong {
                         img ."icon" [src=url!("static/rss.png")]/
                         ~ "RSS-fluo"
                     }
                    ~ "- Aboni la RSS-fluon por novajn bildstriojn"
                }
            }
        }

        h2 { "Aliaj Ligiloj" }
        ul ."links" {
            li { a [href="https://esperaĵo.com"]
                { strong { "Esperaĵo" }
                    ~ "- Esperantaj filmoj kaj libroj" }
            }
        }

        hr/
        br/
        img ."icon-image" [
            src=url!("static/icon.png"),
            alt="La vizaĝo de Garfildo",
            height=400,
        ]/
    }}
}
