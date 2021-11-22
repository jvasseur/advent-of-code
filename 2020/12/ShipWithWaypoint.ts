class ShipWithWaypoint {
    #east: number = 0;
    #north: number = 0;
    #waypointEast: number = 10;
    #waypointNorth: number = 1;

    turnWaypointLeft(degrees: number): void {
        const direction = (degrees + 360) % 360;

        const waypointEast = this.#waypointEast;
        const waypointNorth = this.#waypointNorth;

        switch (direction) {
            case 0:
                // no-op
                break;
            case 90:
                this.#waypointEast = -waypointNorth;
                this.#waypointNorth = waypointEast;
                break;
            case 180:
                this.#waypointEast = -waypointEast;
                this.#waypointNorth = -waypointNorth;
                break;
            case 270:
                this.#waypointEast = waypointNorth;
                this.#waypointNorth = -waypointEast;
                break;
            default:
                throw new Error(`Invalid direction ${direction}`);
        }
    }

    turnWaypointRight(degrees: number): void {
        this.turnWaypointLeft(-degrees);
    }

    moveForward(distance: number): void {
        this.#east += this.#waypointEast * distance;
        this.#north += this.#waypointNorth * distance;
    }

    moveWaypointEast(distance: number): void {
        this.#waypointEast += distance;
    }

    moveWaypointWest(distance: number): void {
        this.#waypointEast -= distance;
    }

    moveWaypointNorth(distance: number): void {
        this.#waypointNorth += distance;
    }

    moveWaypointSouth(distance: number): void {
        this.#waypointNorth -= distance;
    }

    get east(): number {
        return this.#east;
    }

    get north(): number {
        return this.#north;
    }
}

export default ShipWithWaypoint;
