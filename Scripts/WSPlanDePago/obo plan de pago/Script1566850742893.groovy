import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import groovy.json.JsonSlurper as JsonSlurper

GlobalVariable.g_cuit = t_cuit
GlobalVariable.g_idPlan = t_idPlan

WebUI.callTestCase(findTestCase('oauth-server/login obo ok'), [('t_servidor') : GlobalVariable.g_servidor_oauth, ('t_http_username') : 'klk-client'
        , ('t_http_password') : 'klk-secret', ('t_username') : 'rentas-ie-admin', ('t_password') : 'rentasie2812', ('t_onbehalfof') : GlobalVariable.g_cuit], 
    FailureHandling.STOP_ON_FAILURE)

response = WS.sendRequest(findTestObject('WSPlanDePago/c2 plan de pago', [('r_access_token') : GlobalVariable.g_access_token
            , ('r_cuit') : GlobalVariable.g_cuit, ('r_idPlan') : GlobalVariable.g_idPlan]))

WS.verifyResponseStatusCode(response, 200)

WebUI.callTestCase(findTestCase('WSPlanDePago/validar response plan de pago'), [('t_response') : response, ('t_cuit') : GlobalVariable.g_cuit, ('t_idPlan') : GlobalVariable.g_idPlan], FailureHandling.STOP_ON_FAILURE)

