#[allow(
    dead_code,
    non_camel_case_types
)]

pub mod ipopt {

    /// A pointer for anything that is to be passed between the called and individual callback function
    pub type UserDataPtr = *mut core::ffi::c_void;
    
    // Type of all indices of vectors, matrices, etc. (since 3.14.0)
    pub type ipindex = core::ffi::c_int;

    // Type for all number
    pub type ipnumber = core::ffi::c_double;

    /// Pointer to an Ipopt Problem
    pub type IpoptProblem = *mut IpoptProblemInfo;

    // Structure collecting all information about the problem definition and solve statistics etc
    #[repr(C)]
    pub struct IpoptProblemInfo {
        _data: [u8; 0],
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    /// Return codes for the Optimize call for an application
    pub type ApplicationReturnStatus = core::ffi::c_int;

    /// Type defining the callback function for evaluating the value of the objective function.\n\n  Return value should be set to false if there was a problem doing the evaluation.\n\n  See also Ipopt::TNLP::eval_f.
    pub type Eval_F_CB = core::option::Option<
        unsafe extern "C" fn(
            n: ipindex,
            x: *mut ipnumber,
            new_x: bool,
            obj_value: *mut ipnumber,
            user_data: UserDataPtr,
        ) -> bool,
    >;
    
    /// Type defining the callback function for evaluating the gradient of the objective function.\n\n  Return value should be set to false if there was a problem doing the evaluation.\n\n  See also Ipopt::TNLP::eval_grad_f."]
    pub type Eval_Grad_F_CB = core::option::Option<
        unsafe extern "C" fn(
            n: ipindex,
            x: *mut ipnumber,
            new_x: bool,
            grad_f: *mut ipnumber,
            user_data: UserDataPtr,
        ) -> bool,
    >;

    /// Type defining the callback function for evaluating the value of the constraint functions.\n\n  Return value should be set to false if there was a problem doing the evaluation.\n\n  See also Ipopt::TNLP::eval_g."]
    pub type Eval_G_CB = core::option::Option<
        unsafe extern "C" fn(
            n: ipindex,
            x: *mut ipnumber,
            new_x: bool,
            m: ipindex,
            g: *mut ipnumber,
            user_data: UserDataPtr,
        ) -> bool,
    >;
    
    /// Type defining the callback function for evaluating the Jacobian of the constrant functions.\n\n  Return value should be set to false if there was a problem doing the evaluation.\n\n  See also Ipopt::TNLP::eval_jac_g."]
    pub type Eval_Jac_G_CB = core::option::Option<
        unsafe extern "C" fn(
            n: ipindex,
            x: *mut ipnumber,
            new_x: bool,
            m: ipindex,
            nele_jac: ipindex,
            iRow: *mut ipindex,
            jCol: *mut ipindex,
            values: *mut ipnumber,
            user_data: UserDataPtr,
        ) -> bool,
    >;
    
    /// Type defining the callback function for evaluating the Hessian of the Lagrangian function.\n\n  Return value should be set to false if there was a problem doing the evaluation.\n\n  See also Ipopt::TNLP::eval_h."]
    pub type Eval_H_CB = ::std::option::Option<
        unsafe extern "C" fn(
            n: ipindex,
            x: *mut ipnumber,
            new_x: bool,
            obj_factor: ipnumber,
            m: ipindex,
            lambda: *mut ipnumber,
            new_lambda: bool,
            nele_hess: ipindex,
            iRow: *mut ipindex,
            jCol: *mut ipindex,
            values: *mut ipnumber,
            user_data: UserDataPtr,
        ) -> bool,
    >;
    
    /// Type defining the callback function for giving intermediate execution control to the user.\n\n  If set, it is called once per iteration, providing the user\n  with some information on the state of the optimization.\n  This can be used to print some user-defined output.\n  It also gives the user a way to terminate the optimization prematurely.\n  If this method returns false, Ipopt will terminate the optimization.\n\n  See also Ipopt::TNLP::intermediate_callback."]
    pub type Intermediate_CB = ::std::option::Option<
        unsafe extern "C" fn(
            alg_mod: ipindex,
            iter_count: ipindex,
            obj_value: ipnumber,
            inf_pr: ipnumber,
            inf_du: ipnumber,
            mu: ipnumber,
            d_norm: ipnumber,
            regularization_size: ipnumber,
            alpha_du: ipnumber,
            alpha_pr: ipnumber,
            ls_trials: ipindex,
            user_data: UserDataPtr,
        ) -> bool,
    >;   
    
    #[link(name = "ipopt", kind = "dylib")]
    unsafe extern "C" {

        /// Function for creating a new Ipopt Problem object.\n\n  This function returns an object that can be passed to the IpoptSolve call.\n  It contains the basic definition of the optimization problem, such\n  as number of variables and constraints, bounds on variables and\n  constraints, information about the derivatives, and the callback\n  function for the computation of the optimization problem\n  functions and derivatives.  During this call, the options file\n  PARAMS.DAT is read as well.\n\n  If NULL is returned, there was a problem with one of the inputs\n  or reading the options file.\n\n  See also Ipopt::TNLP::get_nlp_info and Ipopt::TNLP::get_bounds_info."]
        pub unsafe fn CreateIpoptProblem(
            n: ipindex,
            x_L: *mut ipnumber,
            x_U: *mut ipnumber,
            m: ipindex,
            g_L: *mut ipnumber,
            g_U: *mut ipnumber,
            nele_jac: ipindex,
            nele_hess: ipindex,
            index_style: ipindex,
            eval_f: Eval_F_CB,
            eval_g: Eval_G_CB,
            eval_grad_f: Eval_Grad_F_CB,
            eval_jac_g: Eval_Jac_G_CB,
            eval_h: Eval_H_CB,
        ) -> IpoptProblem;

        /// Method for freeing a previously created IpoptProblem.\n\n After freeing an IpoptProblem, it cannot be used anymore."]
        pub unsafe fn FreeIpoptProblem(
            ipopt_problem: IpoptProblem
        );

        /// Function for adding a string option.\n\n @return false, if the option could not be set (e.g., if keyword is unknown)"]
        pub unsafe fn AddIpoptStrOption(
            ipopt_problem: IpoptProblem,
            keyword: *mut core::ffi::c_char,
            val: *mut core::ffi::c_char,
        ) -> bool;

        /// Function for adding a Number option.\n\n @return false, if the option could not be set (e.g., if keyword is unknown)"]
        pub unsafe fn AddIpoptNumOption(
            ipopt_problem: IpoptProblem,
            keyword: *mut ::std::os::raw::c_char,
            val: ipnumber,
        ) -> bool;

        /// Function for adding an Integer option.\n\n @return false, if the option  could not be set (e.g., if keyword is unknown)\n@"]
        pub unsafe fn AddIpoptIntOption(
            ipopt_problem: IpoptProblem,
            keyword: *mut ::std::os::raw::c_char,
            val: ipindex,
        ) -> bool;

        /// Function for opening an output file for a given name with given printlevel.\n\n @return false, if there was a problem opening the file."]
        pub unsafe fn OpenIpoptOutputFile(
            ipopt_problem: IpoptProblem,
            file_name: *mut ::std::os::raw::c_char,
            print_level: ::std::os::raw::c_int,
        ) -> bool;
        
        /// Optional function for setting scaling parameter for the NLP.\n\n  This corresponds to the TNLP::get_scaling_parameters method.\n  If the pointers x_scaling or g_scaling are NULL, then no scaling\n  for x resp. g is done."]
        pub unsafe fn SetIpoptProblemScaling(
            ipopt_problem: IpoptProblem,
            obj_scaling: ipnumber,
            x_scaling: *mut ipnumber,
            g_scaling: *mut ipnumber,
        ) -> bool;

        /// Setting a callback function for the \"intermediate callback\" method in the TNLP.\n\n  This gives control back to the user once\n  per iteration.  If set, it provides the user with some\n  information on the state of the optimization.  This can be used\n  to print some user-defined output.  It also gives the user a way\n  to terminate the optimization prematurely.  If the callback\n  method returns false, Ipopt will terminate the optimization.\n  Calling this set method to set the CB pointer to NULL disables\n  the intermediate callback functionality."]
        pub unsafe fn SetIntermediateCallback(
            ipopt_problem: IpoptProblem,
            intermediate_cb: Intermediate_CB,
        ) -> bool;

        /// Function calling the Ipopt optimization algorithm for a problem\n previously defined with CreateIpoptProblem.\n\n @return outcome of the optimization procedure (e.g., success, failure etc)."]
        pub unsafe fn IpoptSolve(
            ipopt_problem: IpoptProblem,
            x: *mut ipnumber,
            g: *mut ipnumber,
            obj_val: *mut ipnumber,
            mult_g: *mut ipnumber,
            mult_x_L: *mut ipnumber,
            mult_x_U: *mut ipnumber,
            user_data: UserDataPtr,
        ) -> ApplicationReturnStatus;

        /// Get primal and dual variable values of the current iterate.\n\n This method can be used to get the values of the current iterate during the intermediate callback set by SetIntermediateCallback().\n The method expects the number of variables (dimension of x), number of constraints (dimension of g(x)),\n and allocated arrays of appropriate lengths as input.\n\n The method translates the x(), c(), d(), y_c(), y_d(), z_L(), and z_U() vectors from Ipopt::IpoptData::curr()\n of the internal NLP representation into the form used by the TNLP.\n For the correspondence between scaled and unscaled solutions, see the detailed description of Ipopt::OrigIpoptNLP.\n If %Ipopt is in restoration mode, it maps the current iterate of restoration %NLP (see Ipopt::RestoIpoptNLP) back to the original TNLP.\n\n If there are fixed variables and fixed_variable_treatment=make_parameter, then requesting z_L and z_U can trigger a reevaluation of\n the Gradient of the objective function and the Jacobian of the constraint functions.\n\n @param ipopt_problem (in) Problem that is currently optimized.\n @param n       (in)  the number of variables \\f$x\\f$ in the problem; can be arbitrary if skipping x, z_L, and z_U\n @param scaled  (in)  whether to retrieve scaled or unscaled iterate\n @param x       (out) buffer to store value of primal variables \\f$x\\f$, must have length at least n; pass NULL to skip retrieving x\n @param z_L     (out) buffer to store the lower bound multipliers \\f$z_L\\f$, must have length at least n; pass NULL to skip retrieving z_L and Z_U\n @param z_U     (out) buffer to store the upper bound multipliers \\f$z_U\\f$, must have length at least n; pass NULL to skip retrieving z_L and Z_U\n @param m       (in)  the number of constraints \\f$g(x)\\f$; can be arbitrary if skipping g and lambda\n @param g       (out) buffer to store the constraint values \\f$g(x)\\f$, must have length at least m; pass NULL to skip retrieving g\n @param lambda  (out) buffer to store the constraint multipliers \\f$\\lambda\\f$, must have length at least m; pass NULL to skip retrieving lambda\n\n @return Whether Ipopt has successfully filled the given arrays\n @since 3.14.0"]
        pub unsafe fn GetIpoptCurrentIterate(
            ipopt_problem: IpoptProblem,
            scaled: bool,
            n: ipindex,
            x: *mut ipnumber,
            z_L: *mut ipnumber,
            z_U: *mut ipnumber,
            m: ipindex,
            g: *mut ipnumber,
            lambda: *mut ipnumber,
        ) -> bool;
        
        /// Get primal and dual infeasibility of the current iterate.\n\n This method can be used to get the violations of constraints and optimality conditions\n at the current iterate during the intermediate callback set by SetIntermediateCallback().\n The method expects the number of variables (dimension of x), number of constraints (dimension of g(x)),\n and allocated arrays of appropriate lengths as input.\n\n The method makes the vectors behind (unscaled_)curr_orig_bounds_violation(), (unscaled_)curr_nlp_constraint_violation(), (unscaled_)curr_dual_infeasibility(),\n (unscaled_)curr_complementarity() from Ipopt::IpoptCalculatedQuantities of the internal NLP representation available into the form used by the TNLP.\n If %Ipopt is in restoration mode, it maps the current iterate of restoration %NLP (see Ipopt::RestoIpoptNLP) back to the original TNLP.\n\n @note If in restoration phase, then requesting grad_lag_x can trigger a call to Eval_F_CB.\n\n @note Ipopt by default relaxes variable bounds (option bound_relax_factor > 0.0).\n   x_L_violation and x_U_violation report the violation of a solution w.r.t. the original unrelaxed bounds.\n   However, compl_x_L and compl_x_U use the relaxed variable bounds to calculate the complementarity.\n\n @param ipopt_problem (in) Problem that is currently optimized.\n @param scaled     (in)  whether to retrieve scaled or unscaled violations\n @param n          (in)  the number of variables \\f$x\\f$ in the problem; can be arbitrary if skipping compl_x_L, compl_x_U, and grad_lag_x\n @param x_L_violation (out) buffer to store violation of original lower bounds on variables (max(orig_x_L-x,0)), must have length at least n; pass NULL to skip retrieving orig_x_L\n @param x_U_violation (out) buffer to store violation of original upper bounds on variables (max(x-orig_x_U,0)), must have length at least n; pass NULL to skip retrieving orig_x_U\n @param compl_x_L  (out) buffer to store violation of complementarity for lower bounds on variables (\\f$(x-x_L)z_L\\f$), must have length at least n; pass NULL to skip retrieving compl_x_L\n @param compl_x_U  (out) buffer to store violation of complementarity for upper bounds on variables (\\f$(x_U-x)z_U\\f$), must have length at least n; pass NULL to skip retrieving compl_x_U\n @param grad_lag_x (out) buffer to store gradient of Lagrangian w.r.t. variables \\f$x\\f$, must have length at least n; pass NULL to skip retrieving grad_lag_x\n @param m          (in)  the number of constraints \\f$g(x)\\f$; can be arbitrary if skipping lambda\n @param nlp_constraint_violation (out) buffer to store violation of constraints \\f$max(g_l-g(x),g(x)-g_u,0)\\f$, must have length at least m; pass NULL to skip retrieving constraint_violation\n @param compl_g    (out) buffer to store violation of complementarity of constraint (\\f$(g(x)-g_l)*\\lambda^+ + (g_l-g(x))*\\lambda^-\\f$, where \\f$\\lambda^+=max(0,\\lambda)\\f$ and \\f$\\lambda^-=max(0,-\\lambda)\\f$ (componentwise)), must have length at least m; pass NULL to skip retrieving compl_g\n\n @return Whether Ipopt has successfully filled the given arrays\n @since 3.14.0"
        pub unsafe fn GetIpoptCurrentViolations(
            ipopt_problem: IpoptProblem,
            scaled: bool,
            n: ipindex,
            x_L_violation: *mut ipnumber,
            x_U_violation: *mut ipnumber,
            compl_x_L: *mut ipnumber,
            compl_x_U: *mut ipnumber,
            grad_lag_x: *mut ipnumber,
            m: ipindex,
            nlp_constraint_violation: *mut ipnumber,
            compl_g: *mut ipnumber,
        ) -> bool;
    }
}
