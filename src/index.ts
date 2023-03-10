import fs from 'fs';


//fs stands for File System

// const data = fs.readFileSync("lines")
//     .toString()
//     .split("/n")
//     .forEach(line => console.log(line))

// const file = fs.readFileSync("lines");

// file.
//     toString().
//     split("\n").
//     filter((_, i) => i % 2 === 0).
//     filter((_, i) => i >= 2 && i < 4).
//     forEach(line => console.log(line));

//     function filter_1(x: number): boolean {
//     return x % 2 === 0;
// }

// function filter_2(x: number): number {
//     return x >= 2 && x < 4;
// }

// // Skipping the split operation
// let a = contents.toString().split("\n");
// let b = [];
// for (let i = 0; i < a.length; ++i) {
//     if (filter_1(a[i])) {
//         b.push(a[i]);
//     }
// }
// let c = [];
// for (let i = 0; i < b.length; ++i) {
//     if (filter_2(i)) {
//         c.push(b[i]);
//     }
// }
// for (let i = 0; i < c.length; ++i) {
//     console.log(c[i]);
// }
// // v8 may optimize some of this away. To what extent, i don't have the faintest clue and neither do you

// //! Enums
// enum Color {
//     Red,
//     Green,
//     Blue,
//     //add yellow and don't receive an error
//     Yellow
// }

// function printColor(color: Color) {
//     switch (color) {
//         case Color.Red:
//             console.log("red");
//             break;
//         case Color.Green:
//             console.log("green");
//             break;
//         case Color.Blue:
//             console.log("blue");
//             break;
//     }
// }

// printColor(Color.Green);


//? Excersie for matching
// type Custom = {
//     name: string,
//     age: number,
// }

// type Item = number | Custom | string;

// function append(items: Item[]) {
//     items.push("hello fem");
// }

// const items: Item[] = [];

// console.log(items);
// append(items);
// console.log(items);

//? Options Exercise
// type Foo = {
//     bar?: string
// }

// const item: Foo = {}
// const item2: Foo = {bar: ""}
// const item3: Foo = {bar: undefined}; // <-- is this valid?

// function multiply(num: number | undefined): number {
//     return (num ?? 0) * 5;
// }

// //? instead of returing 0, if undefined is provided, return undefined else multiply by 5
// function multiply(num: number | undefined): undefined | number {
//     return num === undefined ? undefined : num * 5;
// }

//*Errors

// fs.readFileSync(process.argv[2]).
//     toString().
//     split("\n").
//     forEach(line => console.log(line));

//?lets only print out lines that are numbers and lines that are not, lets print out Line not a number

// fs.readFileSync(process.argv[2]).
//     toString().
//     split("\n").
//     forEach(line => {
//         const v = parseInt(line);
//         if (isNaN(v)) {
//             console.log("Line not a number");
//         } else {
//             console.log(v);
//         }
//     });
//gives us an extra blank at the end of the file, property of node

//*Traits
interface Area {
    area(): Number
}


class Rectangle implements Area {
    constructor(
        public x: number,
        public y: number,
        public width: number,
        public height: number) { }
    area(): number {
        return this.width * this.height;
    }
}

class Circle {
    constructor(public x: number,
        public y: number,
        public radius: number) { }
    area(): number {
        return this.radius * this.radius * Math.PI
    }
}

