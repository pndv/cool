class Main inherits IO { -- test comment with end of line
    -- test comment between dashes --
    main() : SELF_TYPE {
	{
	    out_string((new OBJECT).type_name().substr(4,1))).
	    out_string((isvoid self).type_name().substr(1,3));
	    out_string("\n");
	}
    };
};
