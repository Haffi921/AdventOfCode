import { readInput } from "input";

const zip = (arr1: number[], arr2: number[]) => {
    return arr1.map((_, i) => [arr1[i], arr2[i]]);
}

async function get_input_arrays() {
    const input = (await readInput(2024, 1)).trim().split("\n");
    const left_array: number[] = [];
    const right_array: number[] = [];

    for (const line of input) {
        const [left, right] = line.split("   ").map(s => parseInt(s.trim()));
        left_array.push(left);
        right_array.push(right);
    }
    
    left_array.sort((a, b) => a - b);
    right_array.sort((a, b) => a - b);

    return [left_array, right_array]
}

function part_1(arr_a: number[], arr_b: number[]) {
    const distances: number[] = [];
    for (const [left, right] of zip(arr_a, arr_b)) {
        distances.push(Math.abs(right - left));
    }
    return distances.reduce((a, b) => a + b, 0);
}

function part_2(arr_a: number[], arr_b: number[]) {
    const count_instances = (arr: number[], n: number) => arr.filter(a => a == n).length;
    let similarity_score = 0;
    for (const n of arr_a) {
        similarity_score += n * count_instances(arr_b, n);
    }
    return similarity_score;
}

const [left_array, right_array] = await get_input_arrays();
console.log(`Part 1 ${part_1(left_array, right_array)}`);
console.log(`Part 2 ${part_2(left_array, right_array)}`);
