class CfgVehicles {
    class Man;
    class CAManBase: Man {
        class ACE_SelfActions {
            class GVAR(adminMenu) {
                displayName = "Admin Menu";
                condition = QUOTE(_player call FUNC(isAdmin));
                icon = "a3\3den\data\cfg3den\history\changeattributes_ca.paa";
                statement = QUOTE(call FUNC(openAdminMenu));
            };
        };
    };
};
