package se.lumera.example.payments;

import java.math.BigDecimal;
import java.util.Date;
import java.io.*;

public interface PaymentReceiver {
    /**
     * Called to mark the start of a new payment bundle.
     * @param accountNumber The account number where funds are deposited.
     * @param paymentDate The date at which the funds were/will be deposited on the specified account.
     * @param currency The currency of the payments in the bundle.
     */
    public void startPaymentBundle(String accountNumber, Date paymentDate, String currency);

    /**
     * Called to notify the receiver of a single payment within a bundle.
     * @param amount The payment amount.
     * @param reference The payment reference.
     */
    public void payment(BigDecimal amount, String reference);

    /**
     * Called to mark the end of a payment bundle. This means that there will be no more calls to payment() 
     * for this bundle, and that the receiver can process the bundle.
     */
    public void endPaymentBundle();
}

public class PaymentFileHandlerImpl implements PaymentReceiver {
    private String parseString(String line, int startIdx, int endIdx) {
        return line[startIdx:endIdx].trim()
    }

    private int parseInteger(String line, int startIdx, int endIdx) {
        return Integer.parseInt(line[startIdx:endIdx].trim())
    }

    private BigDecimal parseDecimal(String line, int startIdx, int endIdx, int n) {
        String str = line[startIdx:endIdx].trim()
        str = str[0:n] + "," + str[n:]
        return new BigDecimal(str)
    }

    public void parsePaymentService(File file) {
        BufferedReader br = new BufferedReader(new FileReader(file);

        String line;
        while ((line = br.readLine()) != null) {
//             Write tests here
        }
    }
}

public class CallPaymentFileHandler {
    public static void main(String args[]) {
        PaymentFileHandler = new PaymentFileHandlerImpl();
    }
}
