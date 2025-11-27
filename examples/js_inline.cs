cs js:
    export function formatDate(timestamp) {
        return new Date(timestamp).toLocaleDateString('en-US', {
            year: 'numeric',
            month: 'long',
            day: 'numeric'
        });
    }
    
    export function processArray(arr) {
        return arr
            .filter(x => x > 0)
            .map(x => x * 2)
            .reduce((a, b) => a + b, 0);
    }

mn main():
    timestamp = 1700000000000
    formatted = js.formatDate(timestamp)
    print("Formatted date:", formatted)
    
    numbers = [-2, -1, 0, 1, 2, 3, 4, 5]
    result = js.processArray(numbers)
    print("Processed result:", result)
