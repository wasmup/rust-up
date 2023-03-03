var guessNum = Math.floor(Math.random() * 10) + 1;

function guess(t, n) {
    if (n == guessNum) {
        t.style.background = 'green';
        result1.innerHTML = "You won!";
        return;
    }

    t.style.background = 'red';
    result1.innerHTML = n > guessNum ? "Too high!" : "Too low!";
}