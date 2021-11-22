import { Field, Range } from './types.ts';
import isValidForField from './isValidForField.ts';

const parseInteger = (number: string): number => parseInt(number, 10);

const parseTicket = (ticket: string): number[] => ticket.split(',').map(parseInteger);

const input = await Deno.readTextFile(new URL('input.txt', import.meta.url));

const [fieldsInput, myTicketInput, nearbyTicketsInput] = input.split('\n\n');

const fields: Field[] = fieldsInput.split('\n').map((rangeInput) => {
    const [name, ranges] = rangeInput.split(': ');

    return {
        name,
        ranges: ranges.split(' or ').map((range) => {
            const [min, max] = range.split('-');

            return {
                min: parseInteger(min),
                max: parseInteger(max),
            };
        })
    };
});

const myTicket = parseTicket(myTicketInput.split('\n')[1]);

const tickets = nearbyTicketsInput.split('\n').filter((ticket, index) => index !== 0 && ticket !== '').map(parseTicket);

const isValid = (value: number): boolean => fields.some((field) => isValidForField(value, field));

let invalid = 0;
for (const ticket of tickets) {
    for (const field of ticket) {
        if (!isValid(field)) {
            invalid += field;
        }
    }
}

console.log(invalid);

const filteredTickets = tickets.filter((ticket) => ticket.every(isValid));
const allTickets = [myTicket, ...filteredTickets];

const names = fields.map(({ name }) => name);
const structuredFields = Object.fromEntries(fields.map((field) => [field.name, field]));

let possibleNames = [...Array(myTicket.length)].map(() => names);
const fieldNames: (string|null)[] = [...Array(myTicket.length)].map(() => null);

for (let i = 0; i < myTicket.length; i++) {
    for (const ticket of allTickets) {
        possibleNames[i] = possibleNames[i].filter((possibleName) => isValidForField(ticket[i], structuredFields[possibleName]));
    }
}

while (fieldNames.filter((fieldName) => fieldName === null).length > 0) {
    for (let i = 0; i < myTicket.length; i++) {
        if (fieldNames[i] !== null) {
            continue;
        }

        if (possibleNames[i].length === 1) {
            fieldNames[i] = possibleNames[i][0];

            possibleNames = possibleNames.map((possibleNamesForField) => possibleNamesForField.filter((name) => name !== possibleNames[i][0]));
        }
    }
}

let departure = 1;

for (const [index, fieldName] of (fieldNames as string[]).entries()) {
    if (fieldName.startsWith('departure')) {
        departure *= myTicket[index];
    }
}

console.log(departure);
