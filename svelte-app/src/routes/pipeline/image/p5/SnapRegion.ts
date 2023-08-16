/**
 * p5 class: SnapRegion
 */

import type p5 from 'p5';
import SnapCorner from "./SnapCorner"
import SnapEdge from "./SnapEdge";
import SnapCenter from "./SnapCenter";
import type SnapMember from './SnapMember';
import type { Corners } from '../../../capture/suggestedCorners';


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

    constructor(p5: p5, width: number, height: number, suggestedCorners?: Corners) {

        const topLeftCorner = suggestedCorners?.topLeft || { x: width / 4, y: height / 4 };
        const topRightCorner = suggestedCorners?.topRight || { x: (width / 4) * 3, y: height / 4 };
        const bottomRightCorner = suggestedCorners?.bottomRight || { x: (width / 4) * 3, y: (height / 4) * 3 };
        const bottomLeftCorner = suggestedCorners?.bottomLeft || { x: width / 4, y: (height / 4) * 3 };

        // Initialise all the members
        this.corners = {
            topLeft: new SnapCorner(p5, "topLeft", topLeftCorner),
            topRight: new SnapCorner(p5, "topRight", topRightCorner),
            bottomRight: new SnapCorner(p5, "bottomRight", bottomRightCorner),
            bottomLeft: new SnapCorner(p5, "bottomLeft", bottomLeftCorner)
        };

        this.edges = {
            top: new SnapEdge(p5, "top", this.corners.topLeft, this.corners.topRight),
            left: new SnapEdge(p5, "left", this.corners.topLeft, this.corners.bottomLeft),
            right: new SnapEdge(p5, "right", this.corners.topRight, this.corners.bottomRight),
            bottom: new SnapEdge(p5, "bottom", this.corners.bottomLeft, this.corners.bottomRight)
        };

        this.center = new SnapCenter(p5, "center", this.corners.topLeft, this.corners.topRight, this.corners.bottomRight, this.corners.bottomLeft);


        // Group them in the members container
        this.members = [...Object.values(this.edges), ...Object.values(this.corners), this.center];

        // For the edges and center, some additional setup is required
        this.edges.top.dragAlong(this.corners.topLeft, this.corners.topRight);
        this.edges.left.dragAlong(this.corners.topLeft, this.corners.bottomLeft);
        this.edges.right.dragAlong(this.corners.topRight, this.corners.bottomRight);
        this.edges.bottom.dragAlong(this.corners.bottomLeft, this.corners.bottomRight);

        this.center.dragAlong(...this.members.filter(a => a != this.center));

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
        p5.fill(229, 112, 89, 50);

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
        for (const member of Object.values(this.corners)) {
            if (member.mousePressed(p5)) {
                return;
            }
        }

        for (const member of [...Object.values(this.edges), this.center]) {
            if (member.mousePressed(p5)) {
                return;
            }
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

    /**
     * p5 method: resize
     * @param p5 - p5 instance
     */
    resize(p5: p5, oldWidth: number, oldHeight: number) {

        for (const member of Object.values(this.corners)) {
            member.resize(p5, oldWidth, oldHeight);
        }

    }


}