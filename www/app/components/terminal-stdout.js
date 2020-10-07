export class TerminalStdoutComponent {
    constructor(el) {
        this.el = el;
    }

    println(msg) {
        if (this.el.value) {
            this.el.value += '\n';
        }

        this.el.value += msg;
        this.el.scrollTop = this.el.scrollHeight;
    }

    scrollToTop() {
        this.el.scrollTop = 0;
    }
}
