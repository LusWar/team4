pragma solidity ^0.6;

library SafeMath {

    // +
    function add(uint256 a, uint256 b) internal pure returns (uint256) {
        uint256 c = a + b;

        require(c >= a, "SafeMath: addition overflow");

        return c;
    }

    // -
    function sub(uint256 a, uint256 b, string memory errorMsg) internal pure returns (uint256) {
        require( b <= a,  errorMsg);

        uint256 c = a - b;

        return c;
    }

    // -
    function sub(uint256 a, uint256 b) internal pure returns (uint256) {
        return sub(a, b, "SafeMath: substraction overflow");
    }

    // *
    function mul(uint256 a, uint256 b) internal pure returns (uint256) {
        if (a == 0) {
            return 0;
        }

        uint256 c = a * b;
        require(c / a == b, "SafeMath: multipication overflow");

        return c;
    }

    // /
    function div(uint256 a, uint256 b, string memory errorMsg) internal pure returns (uint256) {
       require( b > 0, errorMsg);

       uint256 c = a / b;

       return c;
    }

    // /
    function div(uint256 a, uint256 b) internal pure returns (uint256) {
        return div(a, b, "SafeMath: division by zero");
    }

    // %
    function mod(uint256 a, uint256 b, string memory errorMsg) internal pure returns (uint256) {
        require(b != 0, errorMsg);

        return a % b;
    }

    // %
    function mod(uint256 a, uint256 b) internal pure returns (uint256) {
        return mod(a, b, "SafeMath: modulo by zero");
    }
}