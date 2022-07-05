class StringUtils {
    /**
     * Method takes strings with placeholders in following format:
     * 'Hello {0} {1} {2}'
     * where 0, 1, 2 are corresponding to index of an argument.
     * 
     * @param s String to format
     * @param args Arguments to replace placeholders
     * @returns Formatted string
     */
    static format<T extends any[]>(s: string, ...args: T): string {
        
        if(s == null) {
            throw "String to format is null";
        }

        const REGEXP = new RegExp("\\{[0-9]+\\}", "gi");
        const matchCount = s.match(REGEXP).length;
        
        if(args.length > matchCount) {
            throw "Too much arguments";
        }
        if(args.length < matchCount) {
            throw "Insufficient number of arguments";
        }

        for(const key in args) {
            s = s.replace(new RegExp("\\{[" + key + "]\\}", "gi"), args[key]);
        }
        return s;
    }
};

export default StringUtils;