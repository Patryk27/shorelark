export class Viewport {
    constructor(el) {
        this.el = el;
    }

    clear() {
        const pixelRatio = window.devicePixelRatio || 1;

        const size = Math.min(
            window.innerWidth - 500,
            window.innerHeight - 50,
        );

        this.el.width = size * pixelRatio;
        this.el.height = size * pixelRatio;
        this.el.style.width = size + 'px';
        this.el.style.height = size + 'px';

        // ---- //

        this.ctxt = this.el.getContext('2d');
        this.ctxt.clearRect(0, 0, this._size(), this._size());
    }

    drawCircle(x, y, radius, style) {
        x *= this._size();
        y *= this._size();
        radius *= this._size();

        this.ctxt.beginPath();
        this.ctxt.arc(x, y, radius, 0.0, 2.0 * Math.PI);
        this.ctxt.fillStyle = style;
        this.ctxt.fill();
    }

    drawArc(x, y, radius, angleFrom, angleTo, style) {
        x *= this._size();
        y *= this._size();
        radius *= this._size();

        this.ctxt.beginPath();
        this.ctxt.arc(x, y, radius, angleFrom, angleTo);
        this.ctxt.strokeStyle = style;
        this.ctxt.lineWidth = 0.002 * this._size();
        this.ctxt.stroke();
    }

    drawTriangle(x, y, size, rotation, style) {
        x *= this._size();
        y *= this._size();
        size *= this._size();
        rotation = -rotation + Math.PI / 2.0;

        this.ctxt.beginPath();

        this.ctxt.moveTo(
            x + Math.sin(rotation) * size * 1.5,
            y + Math.cos(rotation) * size * 1.5,
        );

        this.ctxt.lineTo(
            x + Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size,
            y + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size,
        );

        this.ctxt.lineTo(
            x + Math.sin(rotation + 4.0 / 3.0 * Math.PI) * size,
            y + Math.cos(rotation + 4.0 / 3.0 * Math.PI) * size,
        );

        this.ctxt.lineTo(
            x + Math.sin(rotation) * size * 1.5,
            y + Math.cos(rotation) * size * 1.5,
        );

        this.ctxt.fillStyle = style;
        this.ctxt.fill();
    }

    _size() {
        return this.el.width;
    }
}
