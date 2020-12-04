const input = await Deno.readTextFile('4.txt');

const validateYear = (min: number, max: number, year: string) => {
    if (!/^[0-9]{4}$/.test(year)) {
        return false;
    }
    
    const parsedYear = parseInt(year, 10)
    
    return parsedYear >= min && parsedYear <= max;
}

const validateHeigh = (height: string) => {
    if (/^[0-9]+cm$/.test(height)) {
        const size = parseInt(height.slice(0, -2));
        
        return size >= 150 && size <= 193;
    }

    if (/^[0-9]+in$/.test(height)) {
        const size = parseInt(height.slice(0, -2));
        
        return size >= 59 && size <= 76;
    }

    return false;
}

const passports: Record<string, string>[] = input.split('\n\n').map((passportData) => {
    const passportFields = passportData.split(/[\n ]+/);

    const passport: Record<string, string> = {};

    for (const passportField of passportFields) {
        const [key, value] = passportField.split(':');

        passport[key] = value;
    }

    Object.freeze(passport);

    return passport;
});

console.log(passports.filter((passport) =>
    'byr' in passport &&
    'iyr' in passport &&
    'eyr' in passport &&
    'hgt' in passport &&
    'hcl' in passport &&
    'ecl' in passport &&
    'pid' in passport).length)

console.log(passports.filter((passport) =>
    'byr' in passport && validateYear(1920, 2002, passport.byr) &&
    'iyr' in passport && validateYear(2010, 2020, passport.iyr) &&
    'eyr' in passport && validateYear(2020, 2030, passport.eyr) &&
    'hgt' in passport && validateHeigh(passport.hgt) &&
    'hcl' in passport && /^#[0-9a-f]{6}$/.test(passport.hcl) &&
    'ecl' in passport && ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'].includes(passport.ecl) &&
    'pid' in passport && /^[0-9]{9}$/.test(passport.pid)).length)