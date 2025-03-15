// There are n gas stations along a circular route, where the amount of gas at the ith station is gas[i].

// You have a car with an unlimited gas tank and it costs cost[i] of gas to travel from the ith station to its next (i + 1)th station. You begin the journey with an empty tank at one of the gas stations.

// Given two integer arrays gas and cost, return the starting gas station's index if you can travel around the circuit once in the clockwise direction, otherwise return -1. If there exists a solution, it is guaranteed to be unique.

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let total_gas: i32 = gas.iter().sum();
        let total_cost: i32 = cost.iter().sum();
        if total_gas < total_cost {
            return -1;
        }
        let mut start_index = 0;
        let mut tank = 0;
        for i in 0..gas.len() {
            tank += gas[i] - cost[i];
            if tank < 0 {
                start_index = (i + 1) as i32;
                tank = 0;
            }
        }
        start_index
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]), 3);
    assert_eq!(Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]), -1);
}
