// Seasons palette: 60 colors representing the year divided into ~6-day periods
// Each color captures the essence of that time of year
use crate::color_types::{HexCode, ColorName};

pub const DATA: &[(HexCode, ColorName)] = &[
    // January (Winter depths) - Deep blues and cold whites
    (HexCode::new("#1E3A5F"), ColorName::new("january winter depths deep blue")),       // Jan 1-6
    (HexCode::new("#2B4B7C"), ColorName::new("january winter depths night")),          // Jan 7-12
    (HexCode::new("#C8D8E7"), ColorName::new("january winter depths frost white")),           // Jan 13-18
    (HexCode::new("#4A6FA5"), ColorName::new("january winter depths sky")),           // Jan 19-24
    (HexCode::new("#87CEEB"), ColorName::new("january winter depths morning")),        // Jan 25-31

    // February (Late winter) - Softer blues with hints of change
    (HexCode::new("#6B9BD1"), ColorName::new("february late winter thaw")),         // Feb 1-6
    (HexCode::new("#B0C4DE"), ColorName::new("february late winter mist")),           // Feb 7-12
    (HexCode::new("#4682B4"), ColorName::new("february late winter steel")),          // Feb 13-18
    (HexCode::new("#ADD8E6"), ColorName::new("february late winter sky")),       // Feb 19-24
    (HexCode::new("#E6F3FF"), ColorName::new("february late winter snow")),         // Feb 25-28

    // March (Early spring) - Blues transitioning to greens
    (HexCode::new("#8FBC8F"), ColorName::new("march early spring emergence")),          // Mar 1-6
    (HexCode::new("#9ACD32"), ColorName::new("march early spring first buds")),            // Mar 7-12
    (HexCode::new("#7CFC00"), ColorName::new("march early spring awakening")),      // Mar 13-18
    (HexCode::new("#ADFF2F"), ColorName::new("march early spring green")),           // Mar 19-24
    (HexCode::new("#90EE90"), ColorName::new("march early spring fresh growth")),          // Mar 25-31

    // April (Spring bloom) - Bright greens and fresh colors
    (HexCode::new("#32CD32"), ColorName::new("april spring bloom grass")),           // Apr 1-6
    (HexCode::new("#00FF7F"), ColorName::new("april spring bloom meadow")),         // Apr 7-12
    (HexCode::new("#98FB98"), ColorName::new("april spring bloom new leaves")),            // Apr 13-18
    (HexCode::new("#FFB6C1"), ColorName::new("april spring bloom cherry blossom")),        // Apr 19-24
    (HexCode::new("#DDA0DD"), ColorName::new("april spring bloom lilac")),           // Apr 25-30

    // May (Late spring) - Lush greens with floral touches
    (HexCode::new("#228B22"), ColorName::new("may late spring forest")),            // May 1-6
    (HexCode::new("#FFFF99"), ColorName::new("may late spring dandelion field")),       // May 7-12
    (HexCode::new("#FF69B4"), ColorName::new("may late spring flowers")),        // May 13-18
    (HexCode::new("#40E0D0"), ColorName::new("may late spring rain")),              // May 19-24
    (HexCode::new("#F0E68C"), ColorName::new("may late spring sun")),       // May 25-31

    // June (Early summer) - Bright, warm colors
    (HexCode::new("#FFD700"), ColorName::new("june early summer sunshine")),         // Jun 1-6
    (HexCode::new("#FFFF00"), ColorName::new("june early summer solstice")),       // Jun 7-12
    (HexCode::new("#FFA500"), ColorName::new("june early summer warm day")),         // Jun 13-18
    (HexCode::new("#FF6347"), ColorName::new("june early summer warmth")),          // Jun 19-24
    (HexCode::new("#32CD32"), ColorName::new("june early summer meadow")),           // Jun 25-30

    // July (Peak summer) - Hot, vibrant colors
    (HexCode::new("#FF4500"), ColorName::new("july peak summer heat")),             // Jul 1-6
    (HexCode::new("#DC143C"), ColorName::new("july peak summer intensity")),      // Jul 7-12
    (HexCode::new("#FF8C00"), ColorName::new("july peak summer midsummer")),             // Jul 13-18
    (HexCode::new("#FFA500"), ColorName::new("july peak summer sunset")),           // Jul 19-24
    (HexCode::new("#FFD700"), ColorName::new("july peak summer gold")),           // Jul 25-31

    // August (Late summer) - Deep, rich colors
    (HexCode::new("#FF6347"), ColorName::new("august late summer warmth")),         // Aug 1-6
    (HexCode::new("#CD853F"), ColorName::new("august late summer earth")),     // Aug 7-12
    (HexCode::new("#D2691E"), ColorName::new("august late summer copper")),         // Aug 13-18
    (HexCode::new("#B22222"), ColorName::new("august late summer end")),          // Aug 19-24
    (HexCode::new("#DAA520"), ColorName::new("august late summer harvest")),        // Aug 25-31

    // September (Early autumn) - Warm yellows and early oranges
    (HexCode::new("#FFD700"), ColorName::new("september early autumn gold")),        // Sep 1-6
    (HexCode::new("#FFA500"), ColorName::new("september early autumn equinox")),        // Sep 7-12
    (HexCode::new("#FF8C00"), ColorName::new("september early autumn changing leaves")),       // Sep 13-18
    (HexCode::new("#CD853F"), ColorName::new("september early autumn earth")),       // Sep 19-24
    (HexCode::new("#D2B48C"), ColorName::new("september early autumn harvest time")),          // Sep 25-30

    // October (Peak autumn) - Rich oranges and reds
    (HexCode::new("#FF8C00"), ColorName::new("october peak autumn orange")),        // Oct 1-6
    (HexCode::new("#FF6347"), ColorName::new("october peak autumn fire")),           // Oct 7-12
    (HexCode::new("#B22222"), ColorName::new("october peak autumn red")),           // Oct 13-18
    (HexCode::new("#8B4513"), ColorName::new("october peak autumn brown")),          // Oct 19-24
    (HexCode::new("#A0522D"), ColorName::new("october peak autumn earth")),         // Oct 25-31

    // November (Late autumn) - Browns and muted colors
    (HexCode::new("#8B4513"), ColorName::new("november late autumn brown")),        // Nov 1-6
    (HexCode::new("#A0522D"), ColorName::new("november late autumn fade")),           // Nov 7-12
    (HexCode::new("#CD853F"), ColorName::new("november late autumn dusk")),         // Nov 13-18
    (HexCode::new("#696969"), ColorName::new("november late autumn gray")),         // Nov 19-24
    (HexCode::new("#708090"), ColorName::new("november late autumn end")),          // Nov 25-30

    // December (Early winter) - Cool grays to deep blues
    (HexCode::new("#708090"), ColorName::new("december early winter gray")),         // Dec 1-6
    (HexCode::new("#2F4F4F"), ColorName::new("december early winter approach")),       // Dec 7-12
    (HexCode::new("#191970"), ColorName::new("december early winter night")),        // Dec 13-18
    (HexCode::new("#000080"), ColorName::new("december early winter solstice")),       // Dec 19-24
    (HexCode::new("#1E3A5F"), ColorName::new("december early winter year end")),            // Dec 25-31
];

// This palette represents the natural progression of seasons throughout the year
// Each color is carefully chosen to evoke the feeling and visual character
// of that specific time period, creating a smooth transition through the seasons