class CfgMovesBasic;
class CfgMovesMaleSdr: CfgMovesBasic {
    class States {
        class LadderCivilStatic;
        class LadderCivilUpLoop: LadderCivilStatic {
            speed = 1.5;
        };
    };
};

class CfgAnimation {
    ladderSpeed = 1.5;
};
