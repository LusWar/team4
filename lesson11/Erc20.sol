pragma solidity ^0.6.0;

import "./SafeMath.sol";

contract ERC20 {
    
    using SafeMath for uint256;
    
    uint256 private _totalSupply;
    
    mapping(address => uint256) private _balances;
    
    mapping(address => mapping(address => uint256)) private _allowance;
    
    string private _name;
    
    string private _symbol;
    
    uint8 private _decimals;
    
    event Transfer(address indexed from, address indexed to, uint256 value);
    
    constructor(string memory _myName, string memory _mySymbol, uint8 _myDecimals, uint256 _myTotalSupply) public {
        _name = _myName;
        _symbol = _mySymbol;
        _decimals = _myDecimals;
        _totalSupply = _myTotalSupply;
        _balances[msg.sender] = _totalSupply;
    }
    
    function name() public view returns (string memory) {
        return _name;
    }
    
    function symbol() public view returns (string memory) {
        return _symbol;
    }
        
    function decimals() public view returns (uint8) {
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
    
    function transfer(address to, uint256 value) public returns (bool) {
        _transfer(msg.sender, to, value);
    }
    
    function _transfer(address from, address to, uint256 value) internal returns (bool) {
        require (from != address(0), "ERC20: transfer from zero address");
        require (to != address(0), "ERC20: transfer to zero address");
        
        _balances[from] = _balances[from].sub(value);
        _balances[to] = _balances[to].add(value);
        
        emit Transfer(from, to, value);
    }
    
    function transferFrom(address owner, address to, uint256 value) public returns (bool) {
        require (_allowance[owner][msg.sender] >= value, "ERC20: not allowed to transferFrom");
        return _transfer(owner, to, value);
    }
    
    function approve(address to, uint256 value) public returns (bool) {
        require(_balances[msg.sender] >= value, "ERC20: not enough balance");
        _allowance[msg.sender][to] = value;
        return true;
    }
}