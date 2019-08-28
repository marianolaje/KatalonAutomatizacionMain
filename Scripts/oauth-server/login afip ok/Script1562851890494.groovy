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

response = WS.sendRequest(findTestObject('oauth-server/login afip', [('r_servidor') : GlobalVariable.g_servidor_oauth, ('r_http_username') : 'klk-client'
            , ('r_http_password') : 'klk-secret', ('r_token') : t_token, ('r_sign') : t_sign]))

WS.verifyResponseStatusCode(response, 200)

WebUI.callTestCase(findTestCase('oauth-server/validar login'), [('t_response') : response], FailureHandling.STOP_ON_FAILURE)

response = WS.sendRequest(findTestObject('oauth-server/user', [('r_servidor') : GlobalVariable.g_servidor_oauth, ('r_token') : GlobalVariable.g_access_token]))

WS.verifyResponseStatusCode(response, 200)

