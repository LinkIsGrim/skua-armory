class CfgSoundShaders {
    class Zafir_silencerShot_SoundShader {
        samples[]= {
            {"\A3\Sounds_F\arsenal\weapons\LongRangeRifles\DMR_01_Rahim\DMR01_silencerShot_01",1},
            {"\A3\Sounds_F\arsenal\weapons\LongRangeRifles\DMR_01_Rahim\DMR01_silencerShot_02",1},
            {"\A3\Sounds_F\arsenal\weapons\LongRangeRifles\DMR_01_Rahim\DMR01_silencerShot_03",1}
        };
        volume = "db0";
        range = 150;
        rangeCurve = "closeShotCurve";
    };
    class Zafir_silencerTailForest_SoundShader {
        samples[]= {
            {"\A3\Sounds_F\arsenal\weapons\LongRangeRifles\DMR_01_Rahim\DMR01_silencerTailForest",1}
        };
        volume = "(1-interior/1.4) * forest/3";
        range = 150;
        rangeCurve[]= {
            {0, 1},
            {150, 0.30000001}
        };
        limitation = 1;
    };
    class Zafir_silencerTailHouses_SoundShader {
        samples[]= {
            {"\A3\Sounds_F\arsenal\weapons\LongRangeRifles\DMR_01_Rahim\DMR01_silencerTailHouses",1}
        };
        volume = "(1-interior/1.4) * houses/3";
        range = 150;
        rangeCurve[]= {
            {0, 1},
            {150, 0}
        };
        limitation = 1;
    };
    class Zafir_silencerTailInterior_SoundShader {
        samples[]= {
            {"\A3\Sounds_F\arsenal\weapons\LongRangeRifles\DMR_01_Rahim\DMR01_silencerTailInterior",1}
        };
        volume = "interior";
        range = 150;
        rangeCurve[] = {
            {50, 0.30000001},
            {150, 0}
        };
        limitation = 1;
    };
    class Zafir_silencerTailMeadows_SoundShader {
        samples[]= {
            {"\A3\Sounds_F\arsenal\weapons\LongRangeRifles\DMR_01_Rahim\DMR01_silencerTailMeadows",1}
        };
        volume = "(1-interior/1.4) * (meadows/2 max sea/2)/3";
        range = 150;
        rangeCurve[] = {
            {0, 1},
            {150, 0.30000001}
        };
        limitation = 1;
    };
    class Zafir_silencerTailTrees_SoundShader {
        samples[]= {
            {"\A3\Sounds_F\arsenal\weapons\LongRangeRifles\DMR_01_Rahim\DMR01_silencerTailTrees",1}
        };
        volume = "(1-interior/1.4)*trees/3";
        range = 150;
        rangeCurve[] = {
            {0, 1},
            {150, 0.30000001}
        };
        limitation = 1;
    };
};
