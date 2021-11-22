class Ship {
    #east: number = 0;
    #north: number = 0;
    #direction: number = 0;

    turnLeft(degrees: number): void {
        this.#direction = (this.#direction + degrees) % 360;
    }

    turnRight(degrees: number): void {
        this.#direction = (this.#direction - degrees + 360) % 360;
    }

    moveForward(distance: number): void {
        switch (this.#direction) {
            case 0:
                this.#east += distance;
                break;
            case 90:
                this.#north += distance;
                break;
            case 180:
                this.#east -= distance;
                break;
            case 270:
                this.#north -= distance;
                break;
            default:
                throw new Error(`Invalid direction ${this.#direction}`);
        }
    }

    moveEast(distance: number): void {
        this.#east += distance;
    }

    moveWest(distance: number): void {
        this.#east -= distance;
    }

    moveNorth(distance: number): void {
        this.#north += distance;
    }

    moveSouth(distance: number): void {
        this.#north -= distance;
    }

    get east(): number {
        return this.#east;
    }

    get north(): number {
        return this.#north;
    }
}

export default Ship;
