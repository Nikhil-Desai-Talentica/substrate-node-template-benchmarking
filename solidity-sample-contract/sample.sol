// SPDX-License-Identifier: GPL-3.0
pragma solidity ^0.8.0;

contract Sample {
    string some_str;
    int some_num;

    event SampleEvent();

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

    function fibo(uint n) internal pure returns (uint) {
      if (n < 2) {
        return n;
      }
      return fibo(n-1) + fibo(n-2);
    }

    function fibonacci(uint n) external pure returns (uint) {
      return fibo(n);
    }

    function odd_product(uint n) external pure returns (uint) {
      uint odd_prod_n = 1;
      for(uint i = 1; i <= n; i++) {
        odd_prod_n = odd_prod_n * ((2 * i) - 1);
      }
      return odd_prod_n;
    }

    function triangle_number(uint n) external pure returns (uint) {
      uint tri_num_n = 0;
      for(uint i = 1; i <= n; i++) {
        tri_num_n = tri_num_n + i;
      }
      return tri_num_n;
    }

    function emit_sample_event() external {
      emit SampleEvent();
    }
}
