// Canadian Provinces Palette
// Each province/territory represented by up to 10 most commonly associated colors
// Based on provincial flags, natural features, cultural symbols, and regional characteristics

use crate::define_palette;
use crate::colors_helper::Origin;

define_palette!(
    CanadianProvincesStruct,
    Origin::CanadianProvinces,
    "Canadian Provinces",
    [
        // British Columbia - Pacific coastal blues, forest greens, mountain whites
        ("#003F87", "bc parliament blue"),
        ("#FFFFFF", "bc mountain snow"),
        ("#FFD700", "bc golden sun"),
        ("#2E8B57", "bc coastal forest"),
        ("#4682B4", "bc pacific ocean"),
        ("#228B22", "bc evergreen"),
        ("#FF6347", "bc sunset salmon"),
        ("#87CEEB", "bc sky blue"),
        ("#8FBC8F", "bc rainforest"),
        ("#1E90FF", "bc glacier blue"),

        // Alberta - Prairie golds, oil blacks, mountain blues, cowboy browns
        ("#003C71", "alberta blue"),
        ("#FFD700", "alberta wheat gold"),
        ("#8B4513", "alberta prairie soil"),
        ("#000000", "alberta oil black"),
        ("#DC143C", "alberta red"),
        ("#FFFFFF", "alberta snow peaks"),
        ("#32CD32", "alberta canola green"),
        ("#FF8C00", "alberta sunset orange"),
        ("#4169E1", "alberta sky blue"),
        ("#CD853F", "alberta badlands"),

        // Saskatchewan - Wheat yellows, prairie greens, sky blues
        ("#228B22", "saskatchewan green"),
        ("#FFD700", "saskatchewan wheat"),
        ("#FFFFFF", "saskatchewan lily white"),
        ("#000000", "saskatchewan text black"),
        ("#87CEEB", "saskatchewan big sky"),
        ("#32CD32", "saskatchewan prairie grass"),
        ("#FF6347", "saskatchewan sunset"),
        ("#8B4513", "saskatchewan earth"),
        ("#4682B4", "saskatchewan lake blue"),
        ("#FFFF00", "saskatchewan canola"),

        // Manitoba - Provincial reds, prairie golds, northern blues
        ("#FF0000", "manitoba red"),
        ("#0000FF", "manitoba blue"),
        ("#FFFFFF", "manitoba white"),
        ("#228B22", "manitoba forest green"),
        ("#FFD700", "manitoba grain gold"),
        ("#4682B4", "manitoba lake blue"),
        ("#8B4513", "manitoba earth brown"),
        ("#32CD32", "manitoba spring green"),
        ("#FF8C00", "manitoba autumn orange"),
        ("#800080", "manitoba crocus purple"),

        // Ontario - Red and white, Great Lakes blues, autumn colors
        ("#FF0000", "ontario red"),
        ("#FFFFFF", "ontario white"),
        ("#0000FF", "ontario blue"),
        ("#228B22", "ontario forest"),
        ("#4682B4", "ontario great lakes"),
        ("#FFD700", "ontario autumn gold"),
        ("#8B4513", "ontario maple bark"),
        ("#DC143C", "ontario maple leaf"),
        ("#32CD32", "ontario summer green"),
        ("#FF8C00", "ontario fall orange"),

        // Quebec - Blue and white, fleur-de-lis, maple colors
        ("#0F204B", "quebec blue"),
        ("#FFFFFF", "quebec white"),
        ("#FFD700", "quebec gold"),
        ("#228B22", "quebec forest green"),
        ("#DC143C", "quebec red maple"),
        ("#4682B4", "quebec st lawrence"),
        ("#8B4513", "quebec cabin brown"),
        ("#FF8C00", "quebec autumn"),
        ("#32CD32", "quebec spring"),
        ("#800080", "quebec lilac"),

        // New Brunswick - Forest greens, maritime blues, loyalist colors
        ("#228B22", "nb forest green"),
        ("#4682B4", "nb bay of fundy"),
        ("#FFD700", "nb golden yellow"),
        ("#FF0000", "nb red"),
        ("#FFFFFF", "nb white"),
        ("#0000FF", "nb blue"),
        ("#8B4513", "nb earth brown"),
        ("#FF8C00", "nb autumn orange"),
        ("#32CD32", "nb fiddlehead green"),
        ("#DC143C", "nb lobster red"),

        // Nova Scotia - Saltire blues, maritime colors, Highland tartans
        ("#003F87", "ns saltire blue"),
        ("#FFFFFF", "ns saltire white"),
        ("#FFD700", "ns mayflower gold"),
        ("#228B22", "ns forest green"),
        ("#4682B4", "ns atlantic blue"),
        ("#DC143C", "ns tartan red"),
        ("#8B4513", "ns earth brown"),
        ("#FF8C00", "ns autumn maple"),
        ("#32CD32", "ns spring green"),
        ("#800080", "ns highland purple"),

        // Prince Edward Island - Red soil, potato fields, coastal blues
        ("#DC143C", "pei red soil"),
        ("#FFFFFF", "pei white sand"),
        ("#228B22", "pei potato green"),
        ("#4682B4", "pei coastal blue"),
        ("#FFD700", "pei golden fields"),
        ("#0000FF", "pei ocean blue"),
        ("#8B4513", "pei earth brown"),
        ("#32CD32", "pei summer green"),
        ("#FF8C00", "pei sunset orange"),
        ("#800080", "pei lupine purple"),

        // Newfoundland and Labrador - Ocean blues, iceberg whites, aurora colors
        ("#003F87", "nl blue"),
        ("#FFFFFF", "nl white"),
        ("#FFD700", "nl gold"),
        ("#DC143C", "nl red"),
        ("#4682B4", "nl atlantic"),
        ("#228B22", "nl forest green"),
        ("#00FFFF", "nl iceberg cyan"),
        ("#8B4513", "nl earth brown"),
        ("#FF8C00", "nl sunset"),
        ("#32CD32", "nl spring moss"),

        // Yukon Territory - Gold rush, midnight sun, aurora borealis
        ("#003F87", "yukon blue"),
        ("#FFFFFF", "yukon white"),
        ("#FFD700", "yukon gold"),
        ("#DC143C", "yukon red"),
        ("#228B22", "yukon forest"),
        ("#00FFFF", "yukon aurora cyan"),
        ("#FF00FF", "yukon aurora magenta"),
        ("#8B4513", "yukon earth"),
        ("#32CD32", "yukon summer"),
        ("#4169E1", "yukon midnight"),

        // Northwest Territories - Arctic blues, northern lights, indigenous colors
        ("#003F87", "nwt blue"),
        ("#FFFFFF", "nwt polar white"),
        ("#FFD700", "nwt gold"),
        ("#228B22", "nwt taiga green"),
        ("#00FFFF", "nwt aurora cyan"),
        ("#FF00FF", "nwt aurora magenta"),
        ("#8B4513", "nwt tundra brown"),
        ("#32CD32", "nwt summer green"),
        ("#4682B4", "nwt arctic ocean"),
        ("#FF8C00", "nwt midnight sun"),

        // Nunavut - Arctic whites, polar blues, Inuit cultural colors
        ("#003F87", "nunavut blue"),
        ("#FFFFFF", "nunavut snow white"),
        ("#FFD700", "nunavut inuksuk gold"),
        ("#DC143C", "nunavut red"),
        ("#00FFFF", "nunavut ice cyan"),
        ("#4682B4", "nunavut arctic sea"),
        ("#8B4513", "nunavut stone brown"),
        ("#228B22", "nunavut tundra moss"),
        ("#FF00FF", "nunavut aurora pink"),
        ("#32CD32", "nunavut brief summer"),
    ]
);

// This palette celebrates Canada's provinces and territories through their most
// iconic and culturally significant colors, from BC's Pacific blues to
// Nunavut's Arctic whites, capturing the diverse landscapes and heritage
// of each region across this vast nation.