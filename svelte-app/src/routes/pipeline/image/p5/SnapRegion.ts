/**
 * p5 class: SnapRegion
 */

import type p5 from 'p5';
import SnapCorner from "./SnapCorner"
import SnapEdge from "./SnapEdge";
import SnapCenter from "./SnapCenter";
import type SnapMember from './SnapMember';


export default class SnapRegion {

    // The SnapRegion has 9 member: 4 corners, 4 edges and a center.
    corners: {
        topLeft: SnapCorner,
        topRight: SnapCorner,
        bottomRight: SnapCorner,
        bottomLeft: SnapCorner
    };

    edges: {
        top: SnapEdge,
        left: SnapEdge,
        right: SnapEdge,
        bottom: SnapEdge
    }

    center: SnapCenter;

    // Group all SnapMembers in an array for easy operation
    members: SnapMember[];

    constructor(p5: p5, width: number, height: number) {

        // Initialise all the members
        this.corners = {
            topLeft: new SnapCorner(p5, "topLeft", width / 4, height / 4),
            topRight: new SnapCorner(p5, "topRight", (width / 4) * 3, height / 4),
            bottomRight: new SnapCorner(p5, "bottomRight", (width / 4) * 3, (height / 4) * 3),
            bottomLeft: new SnapCorner(p5, "bottomLeft", width / 4, (height / 4) * 3)
        };

        this.edges = {
            top: new SnapEdge(p5, "top", this.corners.topLeft, this.corners.topRight),
            left: new SnapEdge(p5, "left", this.corners.topLeft, this.corners.bottomLeft),
            right: new SnapEdge(p5, "right", this.corners.topRight, this.corners.bottomRight),
            bottom: new SnapEdge(p5, "bottom", this.corners.bottomLeft, this.corners.bottomRight)
        };

        this.center = new SnapCenter(p5, "center", this.corners.topLeft, this.corners.topRight, this.corners.bottomRight, this.corners.bottomLeft);


        // Group them in the members container
        this.members = [...Object.values(this.corners), ...Object.values(this.edges), this.center];

        // For the edges and center, some additional setup is required
        this.edges.top.dragAlong(this.corners.topLeft, this.corners.topRight);
        this.edges.left.dragAlong(this.corners.topLeft, this.corners.bottomLeft);
        this.edges.right.dragAlong(this.corners.topRight, this.corners.bottomRight);
        this.edges.bottom.dragAlong(this.corners.bottomLeft, this.corners.bottomRight);

        this.center.dragAlong(...this.members.filter(a => a != this.center));

        console.log(this.center.dragAlongMembers)

    }



    private updateEdgesBetweenAllCorners() {
        this.edges.top.reposition(this.corners.topLeft, this.corners.topRight);
        this.edges.left.reposition(this.corners.topLeft, this.corners.bottomLeft);
        this.edges.right.reposition(this.corners.topRight, this.corners.bottomRight);
        this.edges.bottom.reposition(this.corners.bottomLeft, this.corners.bottomRight);

        this.center.reposition(this.corners.topLeft, this.corners.topRight, this.corners.bottomLeft, this.corners.bottomRight);
    }



    /**
     * Method: p5.draw
     */
    draw(p5: p5) {

        // Translucent material that covers the entire region
        p5.noStroke();
        p5.fill(150, 200, 50, 100);

        p5.beginShape();
        for (const corner of Object.values(this.corners)) {
            p5.vertex(corner.pos.x, corner.pos.y);
        }
        p5.endShape(p5.CLOSE);

        // Draw all the members
        for (const member of this.members) {
            member.draw(p5);
        }

        p5.stroke(51);
        p5.noFill();

        p5.ellipse(p5.mouseX, p5.mouseY, 25, 25);


        // If any members are being dragged, handle their effect on other members
        // Address members with a shorter name, for ease of reading
        this.updateEdgesBetweenAllCorners();


    }

    /**
     * Method: mousePressed
     * @param p5 - p5 instance
     */
    mousePressed(p5: p5) {
        // Simply pass on the message to the members
        for (const member of this.members) {
            member.mousePressed(p5);
        }
    }

    /**
     * Method: mouseDragged
     * @param p5 - p5 instance
     */
    mouseDragged(p5: p5) {
        // Simply pass on the message to the members
        for (const member of this.members) {
            member.mouseDragged(p5);
        }
    }

    /**
     * Method: mouseReleased
     * @param p5 - p5 instance
     */
    mouseReleased(p5: p5) {
        // Simply pass on the message to the members
        for (const member of this.members) {
            member.mouseReleased(p5);
        }
    }


}