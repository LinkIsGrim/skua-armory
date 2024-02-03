class CfgSoundSets {
    class Rifle_silencerShot_Base_SoundSet;
    class Rifle_silencerTail_Base_SoundSet;
    class Rifle_silencerInteriorTail_Base_SoundSet;

    class Zafir_silencerShot_SoundSet: Rifle_silencerShot_Base_SoundSet {
        soundShaders[]= {
            "Zafir_Closure_SoundShader",
            "Zafir_silencerShot_SoundShader"
        };
    };
    class Zafir_silencerTail_SoundSet: Rifle_silencerTail_Base_SoundSet {
        soundShaders[]= {
            "Zafir_silencerTailTrees_SoundShader",
            "Zafir_silencerTailForest_SoundShader",
            "Zafir_silencerTailMeadows_SoundShader",
            "Zafir_silencerTailHouses_SoundShader"
        };
    };
    class Zafir_silencerInteriorTail_SoundSet: Rifle_silencerInteriorTail_Base_SoundSet {
        soundShaders[]= {
            "Zafir_silencerTailInterior_SoundShader"
        };
    };
    class acr_silencerShot_SoundSet {
        volumeFactor = 1.2;
    };
};
