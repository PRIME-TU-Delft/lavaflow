/**
 * p5 class: SnapMember
 */

import type p5 from 'p5';

export default class SnapMember {

    id: string;
    pos: p5.Vector;
    dPos: p5.Vector;
    size: p5.Vector;

    // On Drag callback
    dragCallback: (p5: p5, id: string) => void;

    // Other members that should be dragged along with this member
    dragAlongMembers: SnapMember[];

    // State trackers
    isDragged: boolean;

    constructor(p5: p5, id: string, x: number, y: number, dragCallback?: (p5: p5, id: string) => void) {

        this.id = id;

        this.pos = p5.createVector(x, y);
        this.dPos = p5.createVector(0, 0);
        this.size = p5.createVector(10, 10);

        if (dragCallback) {
            this.dragCallback = dragCallback;
        } else {
            this.dragCallback = () => { return };
        }

        // By default, members don't drag along any neighbours
        this.dragAlongMembers = [];

        this.isDragged = false;

    }

    /**
     * Method: attach another member to drag along with this one.
     * @param other - Member to drag along
     */
    dragAlong(...others: SnapMember[]) {
        this.dragAlongMembers.push(...others);
    }

    /**
     * Setter for the dragcallback function (HO)
     * @param dragCallback - callback that's called upon dragging
     */
    public setDragCallback(dragCallback: (p5: p5, id: string) => void) {
        this.dragCallback = dragCallback;
    }

    /**
     * P5 method: draw
     * > Override this method if you want to give this member another shape
     * @param p5 - p5 instance
     */
    public draw(p5: p5): void {

        p5.stroke(0);

        // Whether or not this member is filled, depends on the drag-status
        if (this.isDragged) {
            p5.fill(0);
        } else {
            p5.noFill();
        }

        p5.rectMode(p5.CENTER);
        p5.rect(this.pos.x, this.pos.y, this.size.x, this.size.y);

    }


    /**
     * Method that is called when this member is being dragged by the user
     * > Override this method if you want to add behaviour upon dragging
     * @param p5 - p5 instance
     * @returns void
     */
    protected onDrag(p5: p5): boolean {
        return true;
    }



    /**
     * Method: return true iff mouseX and mouseY are within the container of this member
     * > Override this method if you want to customize the shape of the member
     * @param p5 - p5 instance
     */
    protected mousePressIsWithinContainer(p5: p5): boolean {
        return true &&
            p5.mouseX >= this.pos.x - this.size.x / 2 &&
            p5.mouseX <= this.pos.x + this.size.x / 2 &&
            p5.mouseY >= this.pos.y - this.size.y / 2 &&
            p5.mouseY <= this.pos.y + this.size.y / 2;
    }

    /**
     * p5 method: mousePressed
     * @param p5 - p5 instance
     */
    public mousePressed(p5: p5): void {

        // Check if the press was inside the container of this member
        if (this.mousePressIsWithinContainer(p5)) {
            this.isDragged = true;

            for (const member of this.dragAlongMembers) {
                member.dPos.x = member.pos.x - this.pos.x;
                member.dPos.y = member.pos.y - this.pos.y;
            }
        }

    }

    /**
     * p5 method: mouseDragged
     * @param p5 - p5 instance
     */
    public mouseDragged(p5: p5): void {

        if (this.isDragged) {
            // This member was clicked and dragged, call the activator
            if (this.onDrag(p5)) {
                // If the function returns true, perform the default actions:
                this.dPos.x = p5.mouseX - this.pos.x;
                this.dPos.y = p5.mouseY - this.pos.y;
                this.pos.x = p5.mouseX;
                this.pos.y = p5.mouseY;
                this.dragCallback(p5, this.id);

                // Drag along any members
                for (const member of this.dragAlongMembers) {
                    member.pos.x = this.pos.x + member.dPos.x;
                    member.pos.y = this.pos.y + member.dPos.y;
                }
            }
        }

    }

    /**
     * p5 method: mouseReleased
     * @param p5 - p5 instance
     */
    public mouseReleased(p5: p5): void {
        this.isDragged = false;
    }

}