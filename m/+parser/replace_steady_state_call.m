function string=replace_steady_state_call(string,flag)
% INTERNAL FUNCTION
%

if nargin<2

    flag='';

end

pattern='(\<steady_state|\$)\((\d+)\)';

switch flag

    case 'symbolic'

        new_string='ss_$2';

    otherwise

        new_string='ss($2)';

end

string= regexprep(string,pattern,new_string);

end
