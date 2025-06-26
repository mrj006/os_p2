pub fn help() -> String {
    "
Available Commands:
/fibonacci?num=N
/createfile?name=filename&content=text&repeat=x
/deletefile?name=filename
/reverse?text=abcdef
/toupper?text=abcd
/random?count=n&min=a&max=b
/timestamp
/hash?text=someinput
/simulate?seconds=s&task=name
/sleep?seconds=s
/loadtest?tasks=n&sleep=x
/matrixmult
/countwords
/workers
/help
".to_string()
}
