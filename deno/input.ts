export async function readInput(year: number, day: number, filename: string = "input"): Promise<string> {
    const dayStr = day.toString().padStart(2, '0');
    const path = new URL(`../inputs/${year}/day_${dayStr}/${filename}.txt`, import.meta.url);
    return await Deno.readTextFile(path);
}
  