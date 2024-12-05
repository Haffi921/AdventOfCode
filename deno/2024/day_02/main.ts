import { readInput } from "input";

async function parse_input() {
  return (await readInput(2024, 2))
    .trim()
    .split("\n")
    .map((line) =>
      line
        .trim()
        .split(" ")
        .map((level) => parseInt(level))
    );
}

const is_safe = (report: number[]) => {
  if (report.length < 2) return true;
  const ascending = report[0] < report[1];
  for (const [i, curr] of report.slice(0, report.length - 1).entries()) {
    if (
      ascending && curr >= report[i + 1] ||
      !ascending && curr <= report[i + 1] ||
      Math.abs(curr - report[i + 1]) > 3
    ) {
      return false;
    }
  }
  return true;
};

function part_1(reports: number[][]) {
  return reports.filter(is_safe).length;
}

function part_2(reports: number[][]) {
  const is_safe_with_dampener = (report: number[]) => {
    if (is_safe(report)) return true;
    return report.some((_, i) => is_safe(report.filter((_, j) => j !== i)));
  };

  return reports.filter(is_safe_with_dampener).length;
}

const reports = await parse_input();

console.log(`Part 1: ${part_1(reports)}`);
console.log(`Part 2: ${part_2(reports)}`);
