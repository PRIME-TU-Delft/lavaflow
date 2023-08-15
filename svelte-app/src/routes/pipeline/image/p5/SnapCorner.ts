/**
 * p5 class: SnapCorner
 */

import type p5 from 'p5';
import SnapMember from './SnapMember';
import type SnapEdge from './SnapEdge';
import type SnapCenter from './SnapCenter';

export default class SnapCorner extends SnapMember {

    constructor(p5: p5, id: string, x: number, y: number) {

        super(p5, id, x, y);

    }


    reposition(edge: SnapEdge | SnapCenter, corner: SnapCorner) {
        this.pos.x = corner.pos.x + 2 * (edge.pos.x - corner.pos.x);
        this.pos.y = corner.pos.y + 2 * (edge.pos.y - corner.pos.y);
    }


}