class ACEGVAR(arsenal,sorts) {
    class sortBase;
    class ACE_protectionBallistic: sortBase {
        tabs[] = {{},{}};
    };
    class ACE_protectionExplosive: ACE_protectionBallistic {
        displayName = "Sort by armor";
        tabs[] = {{3,4,6},{}};
    };
};