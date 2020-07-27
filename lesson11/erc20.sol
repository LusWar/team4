pragma solidity ^0.6.0;
import "SafeMath.sol";

contract ERC20 {
    
    using SafeMath for uint256;
    
    // total supply
    uint256 private _totalSupply;
    // balance for each user
    mapping (address => uint256) _balances;
    // allowance
    mapping (address => mapping(address => uint256)) private _allowance;
    string private _name;
    string private _symbol;
    uint8 private _decimals;
    
    event Transfer(address indexed from, address indexed to, uint256 value);
    
    constructor (string memory _myName, string memory _mySymbols, uint8 _myDecimal, uint256 _myTotalSupply) public {
        _name = _myName;
        _symbol = _mySymbols;
        _decimals = _myDecimal;
        _totalSupply = _myTotalSupply;
        _balances[msg.sender] = _totalSupply;
    }
    
    function name() public view returns (string memory) {
        return _name;
    }
    
    function symbol() public view returns (string memory) {
        return _symbol;
    }
    
    function decimal() public view returns (uint8) {
        return _decimals;
    }
    
    function totalSupply() public view returns (uint256) {
        return _totalSupply;
    }
    
    function balanceOf(address account) public view returns (uint256) {
        return _balances[account];
    }
    
    function allowance(address owner, address spender) public view returns (uint256) {
        return _allowance[owner][spender];
    }
    
    function transfer(address from, address to, uint256 value) public returns (bool) {
        return _transfer(from, to, value);
    }
    
    function _transfer(address from, address to, uint256 value) private returns (bool) {
        require(from != address(0), "ERC20: source address is zero");
        require(to != address(0), "ERC20: destination address is zero");
        
        _balances[from] = _balances[msg.sender].sub(value);
        _balances[to] = _balances[to].add(value);
        
        emit Transfer(from, to, value);
        return true;
    }
    
    function transferFrom(address owner, address to, uint256 value) public returns (bool) {
        require(_allowance[owner][to] >= value, "ERC20: allowance is too low");
        return _transfer(owner, to, value);
    }
    
    function approve(address to, uint256 value) public returns (bool) {
        _allowance[msg.sender][to] = value;
    }
        
}