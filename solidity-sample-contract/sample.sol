// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.0;

contract Sample {
    string some_str;
    int some_num;

    constructor () {
        some_str = "Default";
        some_num = 0;
    }

    function set_some_num(int new_num) external {
        some_num = new_num;
    }

    function get_some_num() external view returns (int) {
        return some_num;
    }

    function set_some_str(string memory new_s) external {
        some_str = new_s;
    }

    function get_some_str() external view returns (string memory) {
        return some_str;
    }
}
