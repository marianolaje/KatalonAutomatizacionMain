package internal

import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.main.TestCaseMain


/**
 * This class is generated automatically by Katalon Studio and should not be modified or deleted.
 */
public class GlobalVariable {
     
    /**
     * <p></p>
     */
    public static Object g_access_token
     
    /**
     * <p></p>
     */
    public static Object g_refresh_token
     
    /**
     * <p></p>
     */
    public static Object g_servidor_oauth
     
    /**
     * <p></p>
     */
    public static Object g_servidor_crm
     
    /**
     * <p></p>
     */
    public static Object g_token
     
    /**
     * <p></p>
     */
    public static Object g_timestamp
     
    /**
     * <p></p>
     */
    public static Object g_cuit
     
    /**
     * <p></p>
     */
    public static Object g_planDePago
     
    /**
     * <p></p>
     */
    public static Object g_idPlan
     

    static {
        try {
            def selectedVariables = TestCaseMain.getGlobalVariables("default")
			selectedVariables += TestCaseMain.getGlobalVariables(RunConfiguration.getExecutionProfile())
            selectedVariables += RunConfiguration.getOverridingParameters()
    
            g_access_token = selectedVariables['g_access_token']
            g_refresh_token = selectedVariables['g_refresh_token']
            g_servidor_oauth = selectedVariables['g_servidor_oauth']
            g_servidor_crm = selectedVariables['g_servidor_crm']
            g_token = selectedVariables['g_token']
            g_timestamp = selectedVariables['g_timestamp']
            g_cuit = selectedVariables['g_cuit']
            g_planDePago = selectedVariables['g_planDePago']
            g_idPlan = selectedVariables['g_idPlan']
            
        } catch (Exception e) {
            TestCaseMain.logGlobalVariableError(e)
        }
    }
}
