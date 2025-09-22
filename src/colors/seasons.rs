// Seasons palette: 60 colors representing the year divided into ~6-day periods
// Each color captures the essence of that time of year
use crate::define_palette;
use crate::colors_helper::Origin;

define_palette!(
    SeasonsStruct,
    Origin::Seasons,
    "Seasons",
    [
        // January (Winter depths) - Deep blues and cold whites
        ("#1E3A5F", "january winter depths deep blue"),       // Jan 1-6
        ("#2B4B7C", "january winter depths night"),          // Jan 7-12
        ("#C8D8E7", "january winter depths frost white"),           // Jan 13-18
        ("#4A6FA5", "january winter depths sky"),           // Jan 19-24
        ("#87CEEB", "january winter depths morning"),        // Jan 25-31

        // February (Late winter) - Softer blues with hints of change
        ("#6B9BD1", "february late winter thaw"),         // Feb 1-6
        ("#B0C4DE", "february late winter mist"),           // Feb 7-12
        ("#4682B4", "february late winter steel"),          // Feb 13-18
        ("#ADD8E6", "february late winter sky"),       // Feb 19-24
        ("#E6F3FF", "february late winter snow"),         // Feb 25-28

        // March (Early spring) - Blues transitioning to greens
        ("#8FBC8F", "march early spring emergence"),          // Mar 1-6
        ("#9ACD32", "march early spring first buds"),            // Mar 7-12
        ("#7CFC00", "march early spring awakening"),      // Mar 13-18
        ("#ADFF2F", "march early spring green"),           // Mar 19-24
        ("#90EE90", "march early spring fresh growth"),          // Mar 25-31

        // April (Spring bloom) - Bright greens and fresh colors
        ("#32CD32", "april spring bloom grass"),           // Apr 1-6
        ("#00FF7F", "april spring bloom meadow"),         // Apr 7-12
        ("#98FB98", "april spring bloom new leaves"),            // Apr 13-18
        ("#FFB6C1", "april spring bloom cherry blossom"),        // Apr 19-24
        ("#DDA0DD", "april spring bloom lilac"),           // Apr 25-30

        // May (Late spring) - Lush greens with floral touches
        ("#228B22", "may late spring forest"),            // May 1-6
        ("#FFFF99", "may late spring dandelion field"),       // May 7-12
        ("#FF69B4", "may late spring flowers"),        // May 13-18
        ("#40E0D0", "may late spring rain"),              // May 19-24
        ("#F0E68C", "may late spring sun"),       // May 25-31

        // June (Early summer) - Bright, warm colors
        ("#FFD700", "june early summer sunshine"),         // Jun 1-6
        ("#FFFF00", "june early summer solstice"),       // Jun 7-12
        ("#FFA500", "june early summer warm day"),         // Jun 13-18
        ("#FF6347", "june early summer warmth"),          // Jun 19-24
        ("#32CD32", "june early summer meadow"),           // Jun 25-30

        // July (Peak summer) - Hot, vibrant colors
        ("#FF4500", "july peak summer heat"),             // Jul 1-6
        ("#DC143C", "july peak summer intensity"),      // Jul 7-12
        ("#FF8C00", "july peak summer midsummer"),             // Jul 13-18
        ("#FFA500", "july peak summer sunset"),           // Jul 19-24
        ("#FFD700", "july peak summer gold"),           // Jul 25-31

        // August (Late summer) - Deep, rich colors
        ("#FF6347", "august late summer warmth"),         // Aug 1-6
        ("#CD853F", "august late summer earth"),     // Aug 7-12
        ("#D2691E", "august late summer copper"),         // Aug 13-18
        ("#B22222", "august late summer end"),          // Aug 19-24
        ("#DAA520", "august late summer harvest"),        // Aug 25-31

        // September (Early autumn) - Warm yellows and early oranges
        ("#FFD700", "september early autumn gold"),        // Sep 1-6
        ("#FFA500", "september early autumn equinox"),        // Sep 7-12
        ("#FF8C00", "september early autumn changing leaves"),       // Sep 13-18
        ("#CD853F", "september early autumn earth"),       // Sep 19-24
        ("#D2B48C", "september early autumn harvest time"),          // Sep 25-30

        // October (Peak autumn) - Rich oranges and reds
        ("#FF8C00", "october peak autumn orange"),        // Oct 1-6
        ("#FF6347", "october peak autumn fire"),           // Oct 7-12
        ("#B22222", "october peak autumn red"),           // Oct 13-18
        ("#8B4513", "october peak autumn brown"),          // Oct 19-24
        ("#A0522D", "october peak autumn earth"),         // Oct 25-31

        // November (Late autumn) - Browns and muted colors
        ("#8B4513", "november late autumn brown"),        // Nov 1-6
        ("#A0522D", "november late autumn fade"),           // Nov 7-12
        ("#CD853F", "november late autumn dusk"),         // Nov 13-18
        ("#696969", "november late autumn gray"),         // Nov 19-24
        ("#708090", "november late autumn end"),          // Nov 25-30

        // December (Early winter) - Cool grays to deep blues
        ("#708090", "december early winter gray"),         // Dec 1-6
        ("#2F4F4F", "december early winter approach"),       // Dec 7-12
        ("#191970", "december early winter night"),        // Dec 13-18
        ("#000080", "december early winter solstice"),       // Dec 19-24
        ("#1E3A5F", "december early winter year end"),            // Dec 25-31
    ]
);

// This palette represents the natural progression of seasons throughout the year
// Each color is carefully chosen to evoke the feeling and visual character
// of that specific time period, creating a smooth transition through the seasons