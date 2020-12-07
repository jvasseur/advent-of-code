const input = await Deno.readTextFile('7.txt');

const rules = Object.fromEntries(input.split('\n').filter((line) => line !== '').map((rule) => {
    const [containerText, contentsText] = rule.split(' contain ');

    const container = containerText.replace(/ bags$/, '');

    if (contentsText === 'no other bags.') {
        return [container, []];
    }

    const contents = contentsText.replace(/.$/, '').split(', ').map((contentText) => {
        const { groups: { count, color } } = /^(?<count>\d+) (?<color>[a-z ]+) bags?$/.exec(contentText);

        return { count: parseInt(count, 10), color };
    });

    return [container, contents];
}));

const canContain = (container, bag) => {
    for (const rule of rules[container]) {
        if (rule.color === bag) {
            return true;
        }

        if (canContain(rule.color, bag)) {
            return true;
        }
    }

    return false;
}

let count = 0;

for (const container of Object.keys(rules)) {
    if (canContain(container, 'shiny gold')) {
        count++;
    }
}

console.log(count);

const bagCount = (bag) => {
    let sum = 0;

    for (const rule of rules[bag]) {
        sum += rule.count * (1 + bagCount(rule.color));
    }

    return sum;
}

console.log(bagCount('shiny gold'));
