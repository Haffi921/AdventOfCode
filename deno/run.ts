if (Deno.args.length !== 2) {
    console.error("Usage: deno run run.ts <input> <output>");
    Deno.exit(1);
}

const [year, day] = Deno.args;
const dayPadded = day.padStart(2, "0");

try {
    await import(`./${year}/day_${dayPadded}/main.ts`);
} catch (error) {
    console.error(error);
    Deno.exit(1);
}
