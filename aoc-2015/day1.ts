// --- Day 1: Not Quite Lisp ---
// https://adventofcode.com/2015/day/1
// Key rules:
// - Open Paren -> incriment floor
// - Close Paren -> decriment flor

export async function not_auiet_lisp() {
  const contents = await Deno.readTextFile("./inputs/day1");

  let floor = 0;
  contents.split("").forEach((char, index) => {
    floor += (char == "(") ? 1 : -1;

    // Part two: Find first negative index.
    if (floor < 0) {
      console.log(`Negative Index: ${index + 1}`);
      Deno.exit(0);
    }
  });

  console.log(`Day 1 Not Quiet Lisp: \n - Floor: ${floor}`);
}

not_auiet_lisp();
