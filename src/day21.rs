use super::*;

static SRC_LUT_P1: [u32; 1000] = [
    30, 66, 48, 54, 68, 50, 56, 70, 52, 58, 74, 66, 70, 70, 78, 80, 80, 80, 82, 82, 56, 70, 48, 56, 74, 60, 66, 76, 62,
    68, 68, 78, 72, 54, 82, 76, 66, 84, 78, 68, 76, 82, 78, 78, 68, 72, 72, 80, 82, 82, 58, 74, 64, 64, 72, 50, 58, 76,
    62, 68, 70, 82, 76, 70, 80, 74, 56, 84, 78, 68, 78, 84, 80, 80, 84, 80, 80, 70, 74, 74, 60, 76, 66, 66, 76, 66, 66,
    74, 52, 60, 72, 84, 78, 72, 84, 78, 72, 82, 76, 58, 58, 94, 76, 82, 96, 78, 84, 98, 80, 86, 58, 50, 54, 54, 62, 64,
    64, 64, 66, 66, 62, 76, 54, 62, 80, 66, 72, 82, 68, 74, 68, 78, 72, 54, 82, 76, 66, 84, 78, 68, 70, 76, 72, 72, 62,
    66, 66, 74, 76, 76, 72, 88, 78, 78, 86, 64, 72, 90, 76, 82, 78, 90, 84, 78, 88, 82, 64, 92, 86, 76, 72, 78, 74, 74,
    78, 74, 74, 64, 68, 68, 74, 90, 80, 80, 90, 80, 80, 88, 66, 74, 80, 92, 86, 80, 92, 86, 80, 90, 84, 66, 48, 84, 66,
    72, 86, 68, 74, 88, 70, 76, 70, 62, 66, 66, 74, 76, 76, 76, 78, 78, 48, 62, 40, 48, 66, 52, 58, 68, 54, 60, 62, 72,
    66, 48, 76, 70, 60, 78, 72, 62, 74, 80, 76, 76, 66, 70, 70, 78, 80, 80, 60, 76, 66, 66, 74, 52, 60, 78, 64, 70, 72,
    84, 78, 72, 82, 76, 58, 86, 80, 70, 76, 82, 78, 78, 82, 78, 78, 68, 72, 72, 62, 78, 68, 68, 78, 68, 68, 76, 54, 62,
    74, 86, 80, 74, 86, 80, 74, 84, 78, 60, 44, 80, 62, 68, 82, 64, 70, 84, 66, 72, 62, 54, 58, 58, 66, 68, 68, 68, 70,
    70, 56, 70, 48, 56, 74, 60, 66, 76, 62, 68, 44, 54, 48, 30, 58, 52, 42, 60, 54, 44, 66, 72, 68, 68, 58, 62, 62, 70,
    72, 72, 60, 76, 66, 66, 74, 52, 60, 78, 64, 70, 56, 68, 62, 56, 66, 60, 42, 70, 64, 54, 68, 74, 70, 70, 74, 70, 70,
    60, 64, 64, 62, 78, 68, 68, 78, 68, 68, 76, 54, 62, 58, 70, 64, 58, 70, 64, 58, 68, 62, 44, 60, 96, 78, 84, 98, 80,
    86, 100, 82, 88, 74, 66, 70, 70, 78, 80, 80, 80, 82, 82, 70, 84, 62, 70, 88, 74, 80, 90, 76, 82, 76, 86, 80, 62,
    90, 84, 74, 92, 86, 76, 60, 66, 62, 62, 52, 56, 56, 64, 66, 66, 64, 80, 70, 70, 78, 56, 64, 82, 68, 74, 70, 82, 76,
    70, 80, 74, 56, 84, 78, 68, 72, 78, 74, 74, 78, 74, 74, 64, 68, 68, 74, 90, 80, 80, 90, 80, 80, 88, 66, 74, 80, 92,
    86, 80, 92, 86, 80, 90, 84, 66, 50, 86, 68, 74, 88, 70, 76, 90, 72, 78, 74, 66, 70, 70, 78, 80, 80, 80, 82, 82, 64,
    78, 56, 64, 82, 68, 74, 84, 70, 76, 70, 80, 74, 56, 84, 78, 68, 86, 80, 70, 72, 78, 74, 74, 64, 68, 68, 76, 78, 78,
    50, 66, 56, 56, 64, 42, 50, 68, 54, 60, 64, 76, 70, 64, 74, 68, 50, 78, 72, 62, 76, 82, 78, 78, 82, 78, 78, 68, 72,
    72, 62, 78, 68, 68, 78, 68, 68, 76, 54, 62, 74, 86, 80, 74, 86, 80, 74, 84, 78, 60, 46, 82, 64, 70, 84, 66, 72, 86,
    68, 74, 66, 58, 62, 62, 70, 72, 72, 72, 74, 74, 60, 74, 52, 60, 78, 64, 70, 80, 66, 72, 60, 70, 64, 46, 74, 68, 58,
    76, 70, 60, 64, 70, 66, 66, 56, 60, 60, 68, 70, 70, 58, 74, 64, 64, 72, 50, 58, 76, 62, 68, 46, 58, 52, 46, 56, 50,
    32, 60, 54, 44, 68, 74, 70, 70, 74, 70, 70, 60, 64, 64, 62, 78, 68, 68, 78, 68, 68, 76, 54, 62, 58, 70, 64, 58, 70,
    64, 58, 68, 62, 44, 62, 98, 80, 86, 100, 82, 88, 102, 84, 90, 76, 68, 72, 72, 80, 82, 82, 82, 84, 84, 72, 86, 64,
    72, 90, 76, 82, 92, 78, 84, 78, 88, 82, 64, 92, 86, 76, 94, 88, 78, 76, 82, 78, 78, 68, 72, 72, 80, 82, 82, 72, 88,
    78, 78, 86, 64, 72, 90, 76, 82, 78, 90, 84, 78, 88, 82, 64, 92, 86, 76, 62, 68, 64, 64, 68, 64, 64, 54, 58, 58, 66,
    82, 72, 72, 82, 72, 72, 80, 58, 66, 72, 84, 78, 72, 84, 78, 72, 82, 76, 58, 52, 88, 70, 76, 90, 72, 78, 92, 74, 80,
    76, 68, 72, 72, 80, 82, 82, 82, 84, 84, 66, 80, 58, 66, 84, 70, 76, 86, 72, 78, 72, 82, 76, 58, 86, 80, 70, 88, 82,
    72, 76, 82, 78, 78, 68, 72, 72, 80, 82, 82, 66, 82, 72, 72, 80, 58, 66, 84, 70, 76, 72, 84, 78, 72, 82, 76, 58, 86,
    80, 70, 74, 80, 76, 76, 80, 76, 76, 66, 70, 70, 52, 68, 58, 58, 68, 58, 58, 66, 44, 52, 66, 78, 72, 66, 78, 72, 66,
    76, 70, 52, 48, 84, 66, 72, 86, 68, 74, 88, 70, 76, 68, 60, 64, 64, 72, 74, 74, 74, 76, 76, 62, 76, 54, 62, 80, 66,
    72, 82, 68, 74, 62, 72, 66, 48, 76, 70, 60, 78, 72, 62, 68, 74, 70, 70, 60, 64, 64, 72, 74, 74, 62, 78, 68, 68, 76,
    54, 62, 80, 66, 72, 62, 74, 68, 62, 72, 66, 48, 76, 70, 60, 66, 72, 68, 68, 72, 68, 68, 58, 62, 62, 60, 76, 66, 66,
    76, 66, 66, 74, 52, 60, 48, 60, 54, 48, 60, 54, 48, 58, 52, 34,
];

static SRC_LUT_P2: [u64; 1000] = [
    36698990650,
    80883999266,
    59941760108,
    67297446624,
    80883999268,
    59941760110,
    67297446626,
    80883999270,
    59941760112,
    67297446628,
    95171937380,
    80883999266,
    90897148508,
    88909476672,
    95636614350,
    100705183830,
    98717511994,
    95636614352,
    100705183832,
    98717511996,
    72242026386,
    86627601512,
    59941760108,
    72242026386,
    92371203758,
    74694375192,
    82050061708,
    92371203760,
    74694375194,
    82050061710,
    88417764650,
    95970959866,
    91696170992,
    67297446624,
    101714562112,
    97439773238,
    82050061708,
    101714562114,
    97439773240,
    82050061710,
    95171937382,
    101674419920,
    99387302884,
    97399631048,
    80883999268,
    90897148510,
    88909476674,
    95636614352,
    100705183832,
    98717511996,
    72242026388,
    91839349546,
    80732180762,
    80732180762,
    86627601514,
    59941760110,
    72242026388,
    92371203760,
    74694375194,
    82050061710,
    88417764652,
    101182707900,
    96907919026,
    88087867278,
    95970959868,
    91696170994,
    67297446626,
    101714562114,
    97439773240,
    82050061710,
    95171937384,
    101674419922,
    99387302886,
    97399631050,
    101674419922,
    99387302886,
    97399631050,
    80883999270,
    90897148512,
    88909476676,
    72242026390,
    91839349548,
    80732180764,
    80732180764,
    91839349548,
    80732180764,
    80732180764,
    86627601516,
    59941760112,
    72242026390,
    88417764654,
    101182707902,
    96907919028,
    88087867280,
    101182707902,
    96907919028,
    88087867280,
    95970959870,
    91696170996,
    67297446628,
    72760884850,
    116945893466,
    96003654308,
    103359340824,
    116945893468,
    96003654310,
    103359340826,
    116945893470,
    96003654312,
    103359340828,
    72760884850,
    58472946736,
    68486095978,
    66498424142,
    73225561820,
    78294131300,
    76306459464,
    73225561822,
    78294131302,
    76306459466,
    80786362256,
    95171937382,
    68486095978,
    80786362256,
    100915539628,
    83238711062,
    90594397578,
    100915539630,
    83238711064,
    90594397580,
    87618742168,
    95171937384,
    90897148510,
    66498424142,
    100915539630,
    96640750756,
    81251039226,
    100915539632,
    96640750758,
    81251039228,
    87513499934,
    94015982472,
    91728865436,
    89741193600,
    73225561820,
    83238711062,
    81251039226,
    87978176904,
    93046746384,
    91059074548,
    90594397578,
    110191720736,
    99084551952,
    99084551952,
    104979972704,
    78294131300,
    90594397578,
    110723574950,
    93046746384,
    100402432900,
    97426777490,
    110191720738,
    105916931864,
    97096880116,
    104979972706,
    100705183832,
    76306459464,
    110723574952,
    106448786078,
    91059074548,
    87513499936,
    94015982474,
    91728865438,
    89741193602,
    94015982474,
    91728865438,
    89741193602,
    73225561822,
    83238711064,
    81251039228,
    90594397580,
    110191720738,
    99084551954,
    99084551954,
    110191720738,
    99084551954,
    99084551954,
    104979972706,
    78294131302,
    90594397580,
    97426777492,
    110191720740,
    105916931866,
    97096880118,
    110191720740,
    105916931866,
    97096880118,
    104979972708,
    100705183834,
    76306459466,
    63233013548,
    107418022164,
    86475783006,
    93831469522,
    107418022166,
    86475783008,
    93831469524,
    107418022168,
    86475783010,
    93831469526,
    91906526788,
    77618588674,
    87631737916,
    85644066080,
    92371203758,
    97439773238,
    95452101402,
    92371203760,
    97439773240,
    95452101404,
    63233013548,
    77618588674,
    50932747270,
    63233013548,
    83362190920,
    65685362354,
    73041048870,
    83362190922,
    65685362356,
    73041048872,
    84353331574,
    91906526790,
    87631737916,
    63233013548,
    97650129036,
    93375340162,
    77985628632,
    97650129038,
    93375340164,
    77985628634,
    97650129034,
    104152611572,
    101865494536,
    99877822700,
    83362190920,
    93375340162,
    91387668326,
    98114806004,
    103183375484,
    101195703648,
    77985628632,
    97582951790,
    86475783006,
    86475783006,
    92371203758,
    65685362354,
    77985628632,
    98114806004,
    80437977438,
    87793663954,
    94161366896,
    106926310144,
    102651521270,
    93831469522,
    101714562112,
    97439773238,
    73041048870,
    107458164358,
    103183375484,
    87793663954,
    97650129036,
    104152611574,
    101865494538,
    99877822702,
    104152611574,
    101865494538,
    99877822702,
    83362190922,
    93375340164,
    91387668328,
    77985628634,
    97582951792,
    86475783008,
    86475783008,
    97582951792,
    86475783008,
    86475783008,
    92371203760,
    65685362356,
    77985628634,
    94161366898,
    106926310146,
    102651521272,
    93831469524,
    106926310146,
    102651521272,
    93831469524,
    101714562114,
    97439773240,
    73041048872,
    56663353766,
    100848362382,
    79906123224,
    87261809740,
    100848362384,
    79906123226,
    87261809742,
    100848362386,
    79906123228,
    87261809744,
    78504487096,
    64216548982,
    74229698224,
    72242026388,
    78969164066,
    84037733546,
    82050061710,
    78969164068,
    84037733548,
    82050061712,
    72242026386,
    86627601512,
    59941760108,
    72242026386,
    92371203758,
    74694375192,
    82050061708,
    92371203760,
    74694375194,
    82050061710,
    56663353766,
    64216548982,
    59941760108,
    35543035740,
    69960151228,
    65685362354,
    50295650824,
    69960151230,
    65685362356,
    50295650826,
    84248089342,
    90750571880,
    88463454844,
    86475783008,
    69960151228,
    79973300470,
    77985628634,
    84712766312,
    89781335792,
    87793663956,
    77985628632,
    97582951790,
    86475783006,
    86475783006,
    92371203758,
    65685362354,
    77985628632,
    98114806004,
    80437977438,
    87793663954,
    71415968850,
    84180912098,
    79906123224,
    71086071476,
    78969164066,
    74694375192,
    50295650824,
    84712766312,
    80437977438,
    65048265908,
    84248089344,
    90750571882,
    88463454846,
    86475783010,
    90750571882,
    88463454846,
    86475783010,
    69960151230,
    79973300472,
    77985628636,
    77985628634,
    97582951792,
    86475783008,
    86475783008,
    97582951792,
    86475783008,
    86475783008,
    92371203760,
    65685362356,
    77985628634,
    71415968852,
    84180912100,
    79906123226,
    71086071478,
    84180912100,
    79906123226,
    71086071478,
    78969164068,
    74694375194,
    50295650826,
    72760884852,
    116945893468,
    96003654310,
    103359340826,
    116945893470,
    96003654312,
    103359340828,
    116945893472,
    96003654314,
    103359340830,
    93551305504,
    79263367390,
    89276516632,
    87288844796,
    94015982474,
    99084551954,
    97096880118,
    94015982476,
    99084551956,
    97096880120,
    89276516632,
    103662091758,
    76976250354,
    89276516632,
    109405694004,
    91728865438,
    99084551954,
    109405694006,
    91728865440,
    99084551956,
    96108896544,
    103662091760,
    99387302886,
    74988578518,
    109405694006,
    105130905132,
    89741193602,
    109405694008,
    105130905134,
    89741193604,
    72760884852,
    79263367390,
    76976250354,
    74988578518,
    58472946738,
    68486095980,
    66498424144,
    73225561822,
    78294131302,
    76306459466,
    80786362258,
    100383685416,
    89276516632,
    89276516632,
    95171937384,
    68486095980,
    80786362258,
    100915539630,
    83238711064,
    90594397580,
    87618742170,
    100383685418,
    96108896544,
    87288844796,
    95171937386,
    90897148512,
    66498424144,
    100915539632,
    96640750758,
    81251039228,
    87513499936,
    94015982474,
    91728865438,
    89741193602,
    94015982474,
    91728865438,
    89741193602,
    73225561822,
    83238711064,
    81251039228,
    90594397580,
    110191720738,
    99084551954,
    99084551954,
    110191720738,
    99084551954,
    99084551954,
    104979972706,
    78294131302,
    90594397580,
    97426777492,
    110191720740,
    105916931866,
    97096880118,
    110191720740,
    105916931866,
    97096880118,
    104979972708,
    100705183834,
    76306459466,
    63233013550,
    107418022166,
    86475783008,
    93831469524,
    107418022168,
    86475783010,
    93831469526,
    107418022170,
    86475783012,
    93831469528,
    97118274822,
    82830336708,
    92843485950,
    90855814114,
    97582951792,
    102651521272,
    100663849436,
    97582951794,
    102651521274,
    100663849438,
    84023434202,
    98409009328,
    71723167924,
    84023434202,
    104152611574,
    86475783008,
    93831469524,
    104152611576,
    86475783010,
    93831469526,
    92843485950,
    100396681166,
    96121892292,
    71723167924,
    106140283412,
    101865494538,
    86475783008,
    106140283414,
    101865494540,
    86475783010,
    91906526790,
    98409009328,
    96121892292,
    94134220456,
    77618588676,
    87631737918,
    85644066082,
    92371203760,
    97439773240,
    95452101404,
    63233013550,
    82830336708,
    71723167924,
    71723167924,
    77618588676,
    50932747272,
    63233013550,
    83362190922,
    65685362356,
    73041048872,
    84353331576,
    97118274824,
    92843485950,
    84023434202,
    91906526792,
    87631737918,
    63233013550,
    97650129038,
    93375340164,
    77985628634,
    97650129036,
    104152611574,
    101865494538,
    99877822702,
    104152611574,
    101865494538,
    99877822702,
    83362190922,
    93375340164,
    91387668328,
    77985628634,
    97582951792,
    86475783008,
    86475783008,
    97582951792,
    86475783008,
    86475783008,
    92371203760,
    65685362356,
    77985628634,
    94161366898,
    106926310146,
    102651521272,
    93831469524,
    106926310146,
    102651521272,
    93831469524,
    101714562114,
    97439773240,
    73041048872,
    56663353768,
    100848362384,
    79906123226,
    87261809742,
    100848362386,
    79906123228,
    87261809744,
    100848362388,
    79906123230,
    87261809746,
    83716235130,
    69428297016,
    79441446258,
    77453774422,
    84180912100,
    89249481580,
    87261809744,
    84180912102,
    89249481582,
    87261809746,
    77453774420,
    91839349546,
    65153508142,
    77453774420,
    97582951792,
    79906123226,
    87261809742,
    97582951794,
    79906123228,
    87261809744,
    77453774420,
    85006969636,
    80732180762,
    56333456394,
    90750571882,
    86475783008,
    71086071478,
    90750571884,
    86475783010,
    71086071480,
    78504487098,
    85006969636,
    82719852600,
    80732180764,
    64216548984,
    74229698226,
    72242026390,
    78969164068,
    84037733548,
    82050061712,
    72242026388,
    91839349546,
    80732180762,
    80732180762,
    86627601514,
    59941760110,
    72242026388,
    92371203760,
    74694375194,
    82050061710,
    56663353768,
    69428297016,
    65153508142,
    56333456394,
    64216548984,
    59941760110,
    35543035742,
    69960151230,
    65685362356,
    50295650826,
    84248089344,
    90750571882,
    88463454846,
    86475783010,
    90750571882,
    88463454846,
    86475783010,
    69960151230,
    79973300472,
    77985628636,
    77985628634,
    97582951792,
    86475783008,
    86475783008,
    97582951792,
    86475783008,
    86475783008,
    92371203760,
    65685362356,
    77985628634,
    71415968852,
    84180912100,
    79906123226,
    71086071478,
    84180912100,
    79906123226,
    71086071478,
    78969164068,
    74694375194,
    50295650826,
    72760884854,
    116945893470,
    96003654312,
    103359340828,
    116945893472,
    96003654314,
    103359340830,
    116945893474,
    96003654316,
    103359340832,
    93551305506,
    79263367392,
    89276516634,
    87288844798,
    94015982476,
    99084551956,
    97096880120,
    94015982478,
    99084551958,
    97096880122,
    89276516634,
    103662091760,
    76976250356,
    89276516634,
    109405694006,
    91728865440,
    99084551956,
    109405694008,
    91728865442,
    99084551958,
    96108896546,
    103662091762,
    99387302888,
    74988578520,
    109405694008,
    105130905134,
    89741193604,
    109405694010,
    105130905136,
    89741193606,
    93551305506,
    100053788044,
    97766671008,
    95778999172,
    79263367392,
    89276516634,
    87288844798,
    94015982476,
    99084551956,
    97096880120,
    89276516634,
    108873839792,
    97766671008,
    97766671008,
    103662091760,
    76976250356,
    89276516634,
    109405694006,
    91728865440,
    99084551956,
    96108896546,
    108873839794,
    104599050920,
    95778999172,
    103662091762,
    99387302888,
    74988578520,
    109405694008,
    105130905134,
    89741193604,
    72760884854,
    79263367392,
    76976250356,
    74988578520,
    79263367392,
    76976250356,
    74988578520,
    58472946740,
    68486095982,
    66498424146,
    80786362260,
    100383685418,
    89276516634,
    89276516634,
    100383685418,
    89276516634,
    89276516634,
    95171937386,
    68486095982,
    80786362260,
    87618742172,
    100383685420,
    96108896546,
    87288844798,
    100383685420,
    96108896546,
    87288844798,
    95171937388,
    90897148514,
    66498424146,
    63233013552,
    107418022168,
    86475783010,
    93831469526,
    107418022170,
    86475783012,
    93831469528,
    107418022172,
    86475783014,
    93831469530,
    97118274824,
    82830336710,
    92843485952,
    90855814116,
    97582951794,
    102651521274,
    100663849438,
    97582951796,
    102651521276,
    100663849440,
    84023434204,
    98409009330,
    71723167926,
    84023434204,
    104152611576,
    86475783010,
    93831469526,
    104152611578,
    86475783012,
    93831469528,
    92843485952,
    100396681168,
    96121892294,
    71723167926,
    106140283414,
    101865494540,
    86475783010,
    106140283416,
    101865494542,
    86475783012,
    97118274824,
    103620757362,
    101333640326,
    99345968490,
    82830336710,
    92843485952,
    90855814116,
    97582951794,
    102651521274,
    100663849438,
    84023434204,
    103620757362,
    92513588578,
    92513588578,
    98409009330,
    71723167926,
    84023434204,
    104152611576,
    86475783010,
    93831469526,
    92843485952,
    105608429200,
    101333640326,
    92513588578,
    100396681168,
    96121892294,
    71723167926,
    106140283414,
    101865494540,
    86475783010,
    91906526792,
    98409009330,
    96121892294,
    94134220458,
    98409009330,
    96121892294,
    94134220458,
    77618588678,
    87631737920,
    85644066084,
    63233013552,
    82830336710,
    71723167926,
    71723167926,
    82830336710,
    71723167926,
    71723167926,
    77618588678,
    50932747274,
    63233013552,
    84353331578,
    97118274826,
    92843485952,
    84023434204,
    97118274826,
    92843485952,
    84023434204,
    91906526794,
    87631737920,
    63233013552,
    56663353770,
    100848362386,
    79906123228,
    87261809744,
    100848362388,
    79906123230,
    87261809746,
    100848362390,
    79906123232,
    87261809748,
    83716235132,
    69428297018,
    79441446260,
    77453774424,
    84180912102,
    89249481582,
    87261809746,
    84180912104,
    89249481584,
    87261809748,
    77453774422,
    91839349548,
    65153508144,
    77453774422,
    97582951794,
    79906123228,
    87261809744,
    97582951796,
    79906123230,
    87261809746,
    77453774422,
    85006969638,
    80732180764,
    56333456396,
    90750571884,
    86475783010,
    71086071480,
    90750571886,
    86475783012,
    71086071482,
    83716235132,
    90218717670,
    87931600634,
    85943928798,
    69428297018,
    79441446260,
    77453774424,
    84180912102,
    89249481582,
    87261809746,
    77453774422,
    97051097580,
    85943928796,
    85943928796,
    91839349548,
    65153508144,
    77453774422,
    97582951794,
    79906123228,
    87261809744,
    77453774422,
    90218717670,
    85943928796,
    77123877048,
    85006969638,
    80732180764,
    56333456396,
    90750571884,
    86475783010,
    71086071480,
    78504487100,
    85006969638,
    82719852602,
    80732180766,
    85006969638,
    82719852602,
    80732180766,
    64216548986,
    74229698228,
    72242026392,
    72242026390,
    91839349548,
    80732180764,
    80732180764,
    91839349548,
    80732180764,
    80732180764,
    86627601516,
    59941760112,
    72242026390,
    56663353770,
    69428297018,
    65153508144,
    56333456396,
    69428297018,
    65153508144,
    56333456396,
    64216548986,
    59941760112,
    35543035744,
];

macro_rules! make_lut {
    ($src:expr, $ty:ty) => {{
        let mut dst = [0; 1024];
        let mut d1 = 0;
        while d1 < 10 {
            let mut d2 = 0;
            while d2 < 10 {
                let mut d3 = 0;
                while d3 < 10 {
                    let idx = u32::from_ne_bytes([b'0' + d1 as u8, b'0' + d2 as u8, b'0' + d3 as u8, b'A'])
                        .wrapping_mul(2260763904)
                        >> 22;
                    let code = d1 * 100 + d2 * 10 + d3;
                    dst[idx as usize] = $src[code] * code as $ty;
                    d3 += 1;
                }
                d2 += 1;
            }
            d1 += 1;
        }
        dst
    }};
}

#[inline]
#[repr(align(64))]
unsafe fn inner1(s: &[u8]) -> u32 {
    static LUT: [u32; 1024] = make_lut!(SRC_LUT_P1, u32);
    let ptr = s.as_ptr().cast::<u32>();
    LUT.get_unchecked(ptr.byte_add(0).read_unaligned().wrapping_mul(2260763904) as usize >> 22)
        + LUT.get_unchecked(ptr.byte_add(5).read_unaligned().wrapping_mul(2260763904) as usize >> 22)
        + LUT.get_unchecked(ptr.byte_add(10).read_unaligned().wrapping_mul(2260763904) as usize >> 22)
        + LUT.get_unchecked(ptr.byte_add(15).read_unaligned().wrapping_mul(2260763904) as usize >> 22)
        + LUT.get_unchecked(ptr.byte_add(20).read_unaligned().wrapping_mul(2260763904) as usize >> 22)
}

#[inline]
#[repr(align(64))]
unsafe fn inner2(s: &[u8]) -> u64 {
    static LUT: [u64; 1024] = make_lut!(SRC_LUT_P2, u64);
    let ptr = s.as_ptr();
    let len = s.len();
    let mut sum = 0;
    asm!(
    "20:",
        "imul {idx:e}, [{ptr} + {len} - 5], 2260763904",
        "shr {idx:e}, 22",
        "add {sum}, [{lut} + {idx} * 8]",
        "add {len:e}, -5",
        "jne 20b",
        idx = out(reg) _,
        sum = inout(reg) sum,
        len = inout(reg) len => _,
        ptr = in(reg) ptr,
        lut = in(reg) &LUT,
        options(nostack),
    );
    sum
}

#[inline]
pub fn part1(s: &str) -> u32 {
    unsafe { inner1(s.as_bytes()) }
}

#[inline]
pub fn part2(s: &str) -> u64 {
    unsafe { inner2(s.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn p1() {
        let s = read_to_string("./inputs/21.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part1(s).to_string(), read_to_string("./outputs/21p1.txt").unwrap(),);
    }

    #[test]
    fn p2() {
        let s = read_to_string("./inputs/21.txt").unwrap();
        let s = s.as_str();

        assert_eq!(part2(s).to_string(), read_to_string("./outputs/21p2.txt").unwrap(),);
    }
}
